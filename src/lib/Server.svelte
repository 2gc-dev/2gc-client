<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMount } from 'svelte';
import { user, serverdata, activeTab as globalActiveTab } from '../store';
import type { User, ServerResponse, ServerInfo, MainInfo, ServiceStatus, ServiceInfo, Service } from '../types';
import { writable } from 'svelte/store';
import Auth from '../lib/Auth.svelte';
import { t } from 'svelte-i18n';
import { locale } from 'svelte-i18n';
import toast from 'svelte-french-toast'; // Добавлено для уведомлений

function handleLocaleChange(e: Event) {
  const value = (e.target as HTMLSelectElement).value;
  locale.set(value);
}
import ChatwootWidget from './ChatwootWidget.svelte';
import { getVersion } from "@tauri-apps/api/app";
import { platform } from '@tauri-apps/plugin-os';
import { settings, saveSettings } from '../store/settings';

  let isMacOS = false;

  onMount(async () => {
    // Tauri: platform detection через @tauri-apps/plugin-os
    if ((window as any).__TAURI__) {
      try {
        const { platform } = await import('@tauri-apps/plugin-os');
        isMacOS = (await platform()) === 'macos';
      } catch {
        isMacOS = false;
      }
    } else {
      // Веб-браузер (фолбэк, почти никогда не нужен)
      isMacOS = navigator.userAgent.includes('Mac');
    }
  });





  async function stopCloudflaredProcesses() {
    try {
      await invoke('stop_all_cloudflared_processes_frontend');
      console.log("✅ Запрос на остановку cloudflared процессов отправлен.");
    } catch (error) {
      console.error("❌ Ошибка при отправке запроса на остановку cloudflared:", error);
    }
  }

  function handleDestroy() {
    console.log("👋 Компонент уничтожен (приложение закрывается)");
    stopCloudflaredProcesses();
  }

  onMount(async () => {
  await checkUpdates(); // запуск проверки обновлений

  await fetchMainInfo();

  const answer = await invoke<User>('get_user');
  if (answer) {
    user.set(answer);
  }
});

let appVersion = "";

onMount(async () => {
  appVersion = await getVersion();
}); 

async function checkUpdates() {
  try {
    await invoke("run_updater");
    console.log("✅ Updater запущен");
  } catch (e) {
    console.error("❌ Ошибка запуска обновления:", e);
  }
}

  const activeTabStore = writable('security'); // ВАЖНО: название store
  $: activeTab = $activeTabStore; // реактивное значение

  function setTab(tab: string) {
    activeTabStore.set(tab);
  }


  let showModal = false;
  let username: string = "";
  let password: string = "";
  let additional_data: string = "";
  let remember = false;
  let useSSHKey = false;

  let isEnabled = false;
  let companyInfo: ServerResponse[] = [];
  let filteredCompanyInfo: ServerResponse[] = [];
  let serverInfo: ServerInfo[] = [];
  let mainInfo: MainInfo | null = null;
  let expandedCompanyIds: string[] = [];
  let searchQuery = '';
  let supportMessage = '';
  let currentServer: Service | null = null;
  let currentCompanyId: string = "";

  let isRefreshing = false; // Для анимации обновления списка компаний

  function handleSwitchChange(value: boolean) {
    isEnabled = value;
    console.log('Switch is now', isEnabled ? 'on' : 'off');
  }

async function fetchMainInfo(showToast = false) {
  try {
    isRefreshing = true;
    mainInfo = await invoke('get_servers');
    companyInfo = mainInfo.companys;
    filteredCompanyInfo = companyInfo;

    const savedExpanded = localStorage.getItem('expandedCompanyId');
    if (savedExpanded && companyInfo.find(c => c.id === savedExpanded)) {
      expandedCompanyIds = [savedExpanded];
    } else if (filteredCompanyInfo.length > 0) {
      expandedCompanyIds = [filteredCompanyInfo[0].id];
    }
    if (showToast) {
      toast.success($t('company_list_updated'));
    }
  } catch (e) {
    toast.error($t('company_list_update_error'));
  } finally {
    isRefreshing = false;
  }
}



  

async function connectRdpImmediately() {
  const service = $serverdata.service;
  const companyid = $serverdata.companyid;

  if (!service) return;

  const tunnelid = service.id;
  const protocol = service.protocol;

  const domainPart = additional_data.trim();
  const usernameInput = username;
  const passwordInput = password;

  try {
    const result = await invoke('start_rdp', {
      tunnelid,
      username: usernameInput,
      password: passwordInput,
      domain: domainPart || "",
      remember
    });

    console.log('Результат запуска RDP:', result);

    if (typeof result !== 'string' || !result.startsWith('RDP session started successfully')) {
      console.error('Ошибка запуска RDP:', result);


    }

    // Если результат корректный — просто скрываем модалку без уведомления
    if (typeof result === 'string' && result.startsWith('RDP session started successfully')) {
      showModal = false;
    }

  } catch (error: any) {
    console.error('Исключение при запуске RDP:', error);
  }
}




function singInSsh() {
  console.log('SSH вход с', username, password, additional_data, useSSHKey);
  showModal = false;
}

async function selectFile() {
  console.log('Выбор SSH ключа...');
  // Тут в реальной интеграции будет tauri-plugin-dialog
}


  function getServiceInfo(serviceId: string): ServiceStatus {
    return mainInfo?.service_status[serviceId];
  }

  function getServiceStatusById(serviceId: string): boolean {
    const status = mainInfo?.service_status[serviceId];
    return status?.is_connected ?? false;
  }

function toggleCompany(companyId: string) {
  if (expandedCompanyIds.includes(companyId)) {
    expandedCompanyIds = [];
    localStorage.removeItem('expandedCompanyId');
  } else {
    expandedCompanyIds = [companyId];
    localStorage.setItem('expandedCompanyId', companyId);
  }
}


async function handleConnectClick(server: Service, companyId: string) {
  const serviceState = getServiceInfo(server.id);
  const serviceInfo: ServiceInfo = {
    service: server,
    status: serviceState,
    companyid: companyId
  };
  serverdata.set(serviceInfo);

  // 🟢 Загружаем сохранённые данные перед показом модалки
  try {
    const saved = await invoke<[string, string, string] | null>('get_login_data', { tunnelid: server.id });
    if (saved) {
      username = saved[0];
      password = saved[1];
      additional_data = saved[2];
      remember = true;
    } else {
      username = "";
      password = "";
      additional_data = "";
      remember = false;
    }
  } catch (error) {
    console.error('Ошибка загрузки сохранённых данных:', error);
  }

  showModal = true;
}


  async function toggleConnection(tunnelid: string, protocol: string, isstarted: boolean) {
    if (protocol === 'rdp') {
      await invoke('set_connect_rdp', { tunnelid, isstarted });
    } else if (protocol === 'ssh') {
      await invoke('set_connect_ssh', { tunnelid, isstarted });
    }
    await fetchMainInfo();
  }

async function handleCheckboxChange(event: Event, server: Service, companyid: string) {
  const checkbox = event.target as HTMLInputElement;
  const isstarted = checkbox.checked;

  if (isstarted) {
    try {
      await invoke('set_connect_rdp', { tunnelid: server.id, isstarted: true });
      await fetchMainInfo();
      toast.success($t('tunnel_active')); // Показываем уведомление: туннель активен
    } catch (error) {
      console.error('Ошибка запуска туннеля:', error);
      toast.error($t('tunnel_start_error'));
      checkbox.checked = false;
    }
  } else {
    await toggleConnection(server.id, server.protocol, false);
    toast($t('tunnel_not_active')); // Показываем уведомление: туннель не активен
  }
}



async function cancelConnection() {
  if (currentServer) {
    const input = document.getElementById(`toggle-${currentServer.id}`) as HTMLInputElement;

    if (input) {
      // Плавное отключение
      input.classList.add('smooth-uncheck');
      setTimeout(() => {
        input.checked = false; // снимаем галку через небольшую задержку
        input.classList.remove('smooth-uncheck'); // убираем класс
      }, 300); // задержка должна совпадать с анимацией
    }
    
    await toggleConnection(currentServer.id, currentServer.protocol, false);
  }
  
  showModal = false;
  currentServer = null;
  currentCompanyId = "";
}

function showNotification(message: string) {
  toast(message); // Используем toast вместо alert
}

function handleServerLogin(server: Service, companyid: string) {
  // Переход на страницу подключения
  serverdata.set({ service: server, status: getServiceInfo(server.id), companyid });
  globalActiveTab.set('loginserver');
}

async function signOut(event: Event) {
  await invoke('singout');
  await invoke('clear_storage');
  user.set(null);

  // Установить флаг выхода
  localStorage.setItem('logout_flag', 'true');

  showNotification($t('logout_notification'));

}




  function filteredCompanies() {
    if (!searchQuery) return companyInfo;
    return companyInfo.filter(company => {
      const companyMatch = company.name.toLowerCase().includes(searchQuery.toLowerCase());
      const serverMatch = company.servers.some(server =>
        server.name.toLowerCase().includes(searchQuery.toLowerCase())
      );
      return companyMatch || serverMatch;
    });
  }

  $: filteredCompanyInfo = filteredCompanies();

let activityStats = {
  today: 3,
  week: 12,
  month: 40,
  last: '2025-06-05 13:45'
};

let activityChart = [
  { label: 'Пн', value: 40 },
  { label: 'Вт', value: 60 },
  { label: 'Ср', value: 30 },
  { label: 'Чт', value: 50 },
  { label: 'Пт', value: 70 },
  { label: 'Сб', value: 20 },
  { label: 'Вс', value: 10 }
];

let activitySearch = '';
let activityType = 'all';

let activityHistory = [
  { date: '2025-06-05 13:45', action: 'Вход', server: '-', success: true, type: 'login' },
  { date: '2025-06-05 13:50', action: 'Подключение к RDP', server: 'Server1', success: true, type: 'connect' },
  { date: '2025-06-05 14:00', action: 'Изменение настроек', server: '-', success: true, type: 'settings' },
  { date: '2025-06-05 14:10', action: 'Подключение к SSH', server: 'Server2', success: false, type: 'connect' }
];

$: filteredActivity = activityHistory.filter(item =>
  (activityType === 'all' || item.type === activityType) &&
  (activitySearch === '' ||
    item.action.toLowerCase().includes(activitySearch.toLowerCase()) ||
    item.server.toLowerCase().includes(activitySearch.toLowerCase()) ||
    item.date.includes(activitySearch))
);

let activeSessions = [
  { server: 'Server1', protocol: 'RDP', active: true },
  { server: 'Server2', protocol: 'SSH', active: false }
];

</script>

{#if $user}

<div class="max-w-5xl mx-auto bg-[#171717] rounded-3xl p-6 dashboard-wrapper shadow-inner transition-transform" style="width: 900px; height: 600px; min-width: 900px; min-height: 600px; max-width: 900px; max-height: 600px;">
  <!-- Контент -->
  <div class="mt-6 flex">
    <!-- Левая панель -->
    <div class="w-1/3 pr-4 flex flex-col h-full min-h-[80vh]">
      <!-- Вверх: Поиск + Компании -->
      <div class="flex-1 flex flex-col pr-1">
        <h2 style="margin-bottom: 10px; color: white; display: flex; align-items: center; gap: 8px;">
          {$t('company_servers')}
          <button
            class="refresh-button"
            title="{$t('refresh_company_list')}"
            on:click={() => fetchMainInfo()}
            style="margin-left: 6px;"
            disabled={isRefreshing}
          >
            <i class="fas fa-sync-alt {isRefreshing ? 'fa-spin' : ''}"></i>
          </button>
        </h2>
        <!-- Список компаний -->
        <div class="company-list-scroll flex flex-col gap-1 overflow-y-auto flex-1 pr-2">
          {#if filteredCompanyInfo.length > 0}
            {#each filteredCompanyInfo as company (company.id)}
              <div class="company-block">
                <div
                  class="company-title {expandedCompanyIds.includes(company.id) ? 'active' : ''}"
                  on:click={() => toggleCompany(company.id)}
                >
                  <i class="fas fa-building"></i> {company.name}
                </div>
                <div class="collapsible-content {expandedCompanyIds.includes(company.id) ? 'open' : ''}">
                  <div class="flex flex-col gap-2">
                    {#each company.servers as serv (serv.id)}
                      {#each serv.services as service (service.id)}
                        <div class="button">
                          {#if service.protocol === 'ssh'}
                            <i class="fas fa-terminal bell"></i>
                          {:else if service.protocol === 'rdp'}
                            <i class="fas fa-desktop bell"></i>
                          {:else}
                            <i class="fas fa-server bell"></i>
                          {/if}
                          <div class="w-full text-left">
                            <span class="text-sm">{serv.name}</span>
                          </div>
                          <div class="flex items-center gap-2">
                            <label class="switch">
                              <input
                                type="checkbox"
                                id="toggle-{service.id}"
                                checked={getServiceStatusById(service.id)}
                                on:change={(e) => handleCheckboxChange(e, service, company.id)}
                              />
                              <span class="slider"></span>
                            </label>
                            {#if getServiceStatusById(service.id)}
                              <i
                                class="fa-solid fa-arrow-right-to-bracket text-white cursor-pointer transition-transform hover:scale-125"
                                style="font-size: 1.375rem;"
                                title="{$t('login_title')}"
                                on:click={() => handleConnectClick(service, company.id)}
                              ></i>
                            {:else}
                              <i
                                class="fa-solid fa-arrow-right-to-bracket text-zinc-700 cursor-pointer transition-transform hover:scale-125"
                                style="font-size: 1.375rem;"
                                title="{$t('no_service')}"
                                on:click={() => toast.error($t('tunnel_not_active'))}
                              ></i>
                            {/if}
                          </div>
                        </div>
                      {/each}
                    {/each}
                  </div>
                </div>
              </div>
            {/each}
          {:else}
            <div class="text-gray-400 text-sm">{$t('no_available_servers')}</div>
          {/if}
        </div>
      </div>
      <!-- Разделитель и Профиль/Выход -->
      <div class="mt-auto">
        <div class="border-t border-gray-700 my-4"></div>
        <div>
          <div class="font-semibold text-gray-400 text-sm">{$user?.name ?? 'Пользователь'}</div>
        </div>
        <div class="mt-2 text-gray-400 text-sm cursor-pointer hover:text-white transition-colors flex items-center gap-2" on:click={signOut}>
          <i class="fas fa-sign-out-alt"></i> {$t('logout_button')}
        </div>
        <div class="mt-2 flex flex-col items-start space-y-1">
          <p class="text-gray-500 text-xs">{$t('version')} {appVersion}</p>

          <button
            on:click={checkUpdates}
            class="flex items-center text-gray-500 text-xs hover:text-white transition-colors"
          >
            <!-- Иконка обновления (Heroicon) -->
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-4 w-4 mr-1"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 4v6h6M20 20v-6h-6M4 14a8 8 0 0114.32-5.906M20 10a8 8 0 01-14.32 5.906"
              />
            </svg>
            {$t('checkUpdates')}
          </button>
        </div>
      </div>
    </div>
    <!-- Правая панель -->
    <div class="w-2/3 pl-6" style="margin-top: 20px;">
      <!-- Вкладки -->
      <div class="flex space-x-4 border-b border-gray-700 mb-6">
        <div
          class="inline-flex items-center py-2 px-1 text-sm font-semibold cursor-pointer border-b-2 transition-colors duration-200 { $activeTabStore === 'security' ? 'text-blue-400 border-blue-400' : 'text-gray-400 border-transparent' }"
          on:click={() => setTab('security')}
        >
          {$t('tab_security')}
        </div>
        <div
          class="inline-flex items-center py-2 px-1 text-sm font-semibold cursor-pointer border-b-2 transition-colors duration-200 { $activeTabStore === 'activity' ? 'text-blue-400 border-blue-400' : 'text-gray-400 border-transparent' }"
          on:click={() => setTab('activity')}
        >
          {$t('tab_activity')}
        </div>
        <div
          class="inline-flex items-center py-2 text-sm px-1 font-semibold cursor-pointer border-b-2 transition-colors duration-200 { $activeTabStore === 'support' ? 'text-blue-400 border-blue-400' : 'text-gray-400 border-transparent' }"
          on:click={() => setTab('support')}
        >
          {$t('tab_support')}
        </div>
        <div
          class="inline-flex items-center py-2 px-1 text-sm font-semibold cursor-pointer border-b-2 transition-colors duration-200 { $activeTabStore === 'settings' ? 'text-blue-400 border-blue-400' : 'text-gray-400 border-transparent' }"
          on:click={() => setTab('settings')}
        >
          {$t('tab_settings')}
        </div>
      </div>
      <!-- Контент в зависимости от выбранной вкладки -->
      <div class="mt-6">
        {#if $activeTabStore === 'security'}
          <!-- From Uiverse.io by Praashoo7 --> 
<div class="card">
  <img
    src="https://uiverse.io/astronaut.png"
    alt=""
    class="image"
  />
<div class="text-white font-semibold">
  {$t('security_greeting')}, {$user?.name ?? 'Гость'}{$t('security_greeting_suffix')}
</div>




  <div class="icons">
    <a class="instagram">

    </a>
    <a class="x">
    </a>
    <a class="discord">
    </a>
  </div>
</div>

        {:else if $activeTabStore === 'activity'}
          <div class="activity-container">
    <!-- Статистика -->
    <div class="activity-section">
      <h3 class="activity-title">{$t('activity_stats')}</h3>
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon"><i class="fas fa-plug"></i></div>
          <div class="stat-info">
            <div class="stat-value">{activityStats.today}</div>
            <div class="stat-label">{$t('connections_today')}</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon"><i class="fas fa-calendar-week"></i></div>
          <div class="stat-info">
            <div class="stat-value">{activityStats.week}</div>
            <div class="stat-label">{$t('connections_week')}</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon"><i class="fas fa-calendar-alt"></i></div>
          <div class="stat-info">
            <div class="stat-value">{activityStats.month}</div>
            <div class="stat-label">{$t('connections_month')}</div>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-icon"><i class="fas fa-clock"></i></div>
          <div class="stat-info">
            <div class="stat-value">{activityStats.last}</div>
            <div class="stat-label">{$t('last_connection')}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- График активности (заглушка) -->
    <div class="activity-section">
      <h3 class="activity-title">{$t('activity_chart')}</h3>
      <div class="chart-container">
        <div class="chart-placeholder">
          {#each activityChart as bar}
            <div class="chart-bar" style="height: {bar.value}px" title={bar.label}></div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Фильтры и поиск -->
    <div class="activity-section">
      <h3 class="activity-title">{$t('activity_history')}</h3>
      <div class="history-filters">
        <input class="history-search" type="text" placeholder="{$t('search_activity')}" bind:value={activitySearch} />
        <select class="history-select" bind:value={activityType}>
          <option value="all">{$t('all_types')}</option>
          <option value="connect">{$t('activity_connect')}</option>
          <option value="settings">{$t('activity_settings')}</option>
          <option value="login">{$t('activity_login')}</option>
        </select>
      </div>
      <div class="history-table">
        <table>
          <thead>
            <tr>
              <th>{$t('date')}</th>
              <th>{$t('action')}</th>
              <th>{$t('server')}</th>
              <th>{$t('result')}</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredActivity as item}
              <tr>
                <td>{item.date}</td>
                <td>{item.action}</td>
                <td>{item.server}</td>
                <td>
                  <span class="status-badge {item.success ? 'success' : 'error'}">
                    {item.success ? $t('success') : $t('error')}
                  </span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- Статусы -->
    <div class="activity-section">
      <h3 class="activity-title">{$t('current_sessions')}</h3>
      <ul>
        {#each activeSessions as session}
          <li>
            <span class="status-badge {session.active ? 'success' : 'error'}">
              {session.active ? $t('active') : $t('inactive')}
            </span>
            {session.server} ({session.protocol})
          </li>
        {/each}
      </ul>
    </div>
  </div>
        {:else if $activeTabStore === 'support'}
         {#if $activeTabStore === 'support'}

          <!-- Добавляем чат сразу под формой -->
        <div class="mt-6">
          <ChatwootWidget />
        </div>

        <!-- FAQ ниже чата поддержки -->
        <div class="support-section">
  <h3 class="support-title">{$t('frequently_asked_questions')}</h3>
  <!-- Общие вопросы -->
  <div class="faq-category">
    <div class="faq-category-title">{$t('faq.general.title')}</div>
    <ul class="faq-list">
      {#each $t('faq.general.items') as item (item.id)}
        <li class="faq-item">
          <details>
            <summary class="faq-question">{item.question}</summary>
            <div class="faq-answer">{item.answer}</div>
          </details>
        </li>
      {/each}
    </ul>
  </div>
  <!-- Технические вопросы -->
  <div class="faq-category">
    <div class="faq-category-title">{$t('faq.technical.title')}</div>
    <ul class="faq-list">
      {#each $t('faq.technical.items') as item (item.id)}
        <li class="faq-item">
          <details>
            <summary class="faq-question">{item.question}</summary>
            <div class="faq-answer">{item.answer}</div>
          </details>
        </li>
      {/each}
    </ul>
  </div>
  <!-- Безопасность -->
  <div class="faq-category">
    <div class="faq-category-title">{$t('faq.security.title')}</div>
    <ul class="faq-list">
      {#each $t('faq.security.items') as item (item.id)}
        <li class="faq-item">
          <details>
            <summary class="faq-question">{item.question}</summary>
            <div class="faq-answer">{item.answer}</div>
          </details>
        </li>
      {/each}
    </ul>
  </div>
</div>
  {/if}

        {:else if $activeTabStore === 'settings'}
          <div class="settings-container">
<!-- Настройки интерфейса -->
<div class="settings-section">
  <h3 class="settings-title">{$t('interface_settings')}</h3>
  <div class="settings-group">
    <label class="settings-label">
      <select
        class="settings-select"
        bind:value={$locale}
        on:change={handleLocaleChange}
      >
        <option value="ru">Русский</option>
        <option value="en">English</option>
      </select>
    </label>
  </div>
</div>

<!-- Настройки уведомлений -->
<div class="settings-section">
  <h3 class="settings-title">{$t('notification_settings')}</h3>
  <div class="settings-group">
    <label class="settings-label">
      <span>{$t('enable_notifications')}</span>
      <label class="switch">
        <input type="checkbox" bind:checked={$settings.enableNotifications}>
        <span class="slider"></span>
      </label>
    </label>
    <label class="settings-label">
      <span>{$t('connection_notifications')}</span>
      <label class="switch">
        <input type="checkbox" bind:checked={$settings.connectionNotifications}>
        <span class="slider"></span>
      </label>
    </label>
  </div>
</div>

<!-- Настройки приложения -->
<div class="settings-section">
  <h3 class="settings-title">{$t('application_settings')}</h3>
  <div class="settings-group">
    <label class="settings-label">
      <span>{$t('auto_update')}</span>
      <label class="switch">
        <input type="checkbox" bind:checked={$settings.autoUpdate}>
        <span class="slider"></span>
      </label>
    </label>
    <label class="settings-label">
      <span>{$t('start_with_system')}</span>
      <label class="switch">
        <input type="checkbox" bind:checked={$settings.startWithSystem}>
        <span class="slider"></span>
      </label>
    </label>
    <label class="settings-label">
      <span>{$t('save_credentials')}</span>
      <label class="switch">
        <input type="checkbox" bind:checked={$settings.saveCredentials}>
        <span class="slider"></span>
      </label>
    </label>
  </div>
</div>
  <div class="settings-actions">
    <button
      class="settings-button save"
      on:click={async () => {
        await saveSettings();
        toast.success($t('settings_saved'));
      }}
    >
      {$t('save_settings')}
    </button>
  </div>
</div>
        {/if}
      </div>
    </div> <!-- <-- Закрываем w-2/3 -->
  </div>
</div>

{#if showModal}
  <div class="modal-overlay" on:click={() => showModal = false}>
    <div class="modal-content" on:click|stopPropagation>

      <!-- Кнопка закрытия -->
      <button class="modal-close" on:click={() => showModal = false}>✖</button>

      <!-- Заголовок с меньшим шрифтом -->
      <h2 class="modal-title small-title">{$t('connect_to_server')}</h2>

      <!-- Контент -->
      <div class="space-y-4">
        {#if $serverdata.service.protocol == 'rdp'}
          <input type="text" placeholder="{$t('username_placeholder')}" bind:value={username} class="modal-input">
          {#if !isMacOS}
            <input type="password" placeholder="{$t('password_input_placeholder')}" bind:value={password} class="modal-input">
          {/if}
          <input type="text" placeholder="{$t('domain_placeholder')}" bind:value={additional_data} class="modal-input">

          <small class="text-gray-400 text-xs">{$t('leave_blank_if_no_domain')}</small>


          <div class="modal-checkbox">
            <input type="checkbox" bind:checked={remember} id="rememberRdp">
            <label for="rememberRdp">{$t('remember_checkbox')}</label>
          </div>

<button on:click={connectRdpImmediately} class="modal-button">
  {$t('connect_rdp_button')}
</button>

        {:else if $serverdata.service.protocol == 'ssh'}
<input type="text" placeholder="{$t('username_placeholder')}" bind:value={username} class="modal-input">
<input type="password" placeholder="{$t('password_input_placeholder')}" bind:value={password} class="modal-input">

          <div class="modal-checkbox">
            <input type="checkbox" bind:checked={useSSHKey} id="useSSHKey">
            <label for="useSSHKey">{$t('use_ssh_key_checkbox')}</label>
          </div>

          {#if useSSHKey}
<button type="button" on:click={selectFile} class="modal-upload-button">
  {#if additional_data !== ""}{additional_data}{:else}{$t('select_ssh_key_button')}{/if}
</button>
          {/if}

          <div class="modal-checkbox">
            <input type="checkbox" bind:checked={remember} id="rememberSsh">
            <label for="rememberSsh">{$t('remember_checkbox')}</label>
          </div>

          <button on:click={singInSsh} class="modal-button">
            {$t('connect_ssh_button')}
          </button>
        {/if}
      </div>

    </div> 
  </div>

{/if}

{:else}
  <Auth />
{/if}
<style>
/* Заголовок компаний */
h2 {
  font-size: 0.875rem;
  letter-spacing: 0.05em;
  color: #a0a0a0;
  text-transform: uppercase;
  margin-bottom: 8px;
}

/* Одна компания */
.company-item {
  background-color: #202020;
  border: 1px solid #2e2e2e;
  border-radius: 10px;
  padding: 8px 12px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.company-item:hover {
  background-color: #2a2a2a;
  border-color: #3a3a3a;
}

/* Название компании */
.company-name {
  font-size: 0.85rem;
  color: #e0e0e0;
  font-weight: 500;
}

/* Список серверов */
.server-list {
  margin-top: 4px;
  padding-left: 10px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

/* Одна кнопка сервера */
.button {
  height: 36px; /* меньше высота */
  gap: 10px;
  padding: 0 12px;
  border-radius: 8px;
  font-size: 0.75rem; /* меньше шрифт */
}



/* Кнопка сервера */
.button {
  width: 100%;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 15px;
  padding: 0px 20px;
  background-color: #1f1f1f;
  border-radius: 15px;
  color: white;
  border: 1px solid #2e2e2e;
  position: relative;
  cursor: pointer;
  transition-duration: .2s;
  overflow: hidden;
}


.bell path {
  fill: rgb(0, 206, 62);
}

.arrow {
  position: absolute;
  right: 10px;
  width: 50px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Анимация стрелки */
.button:hover {
  background-color: rgb(77, 77, 77);
  transition-duration: .2s;
}





/* Стилизация скролла */
.flex.flex-col.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.flex.flex-col.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: #555;
  border-radius: 10px;
}

.flex.flex-col.overflow-y-auto::-webkit-scrollbar-track {
  background-color: #2e2e2e;
}

/* From Uiverse.io by Praashoo7 */ 
/* HOLD THE ASTRONAUT */

.card {
  position: relative;

  height: 25em;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: #171717;
  color: white;
  font-weight: bold;
  padding: 1em 2em 1em 1em;
  border-radius: 20px;
  overflow: hidden;
  z-index: 1;
  row-gap: 1em;
}
.card img {
  width: 12em;
  margin-right: 1em;
  animation: move 10s ease-in-out infinite;
  z-index: 5;
}
.image:hover {
  cursor: -webkit-grab;
  cursor: grab;
}

.icons svg {
  width: 20px;
  height: 20px;
}

.card::before {
  content: "";
  position: absolute;
  width: 100%;
  height: 100%;
  inset: -3px;
  border-radius: 10px;
  transform: translate(-5px, 250px);
  transition: 0.4s ease-in-out;
  z-index: -1;
}
.card:hover::before {
  width: 150%;
  height: 100%;
  margin-left: -4.25em;
}
.card::after {
  content: "";
  position: absolute;
  inset: 2px;
  border-radius: 20px;
  background: rgb(23, 23, 23, 0.7);
  transition: all 0.4s ease-in-out;
  z-index: -1;
}

.heading {
  z-index: 2;
  transition: 0.4s ease-in-out;
}
.card:hover .heading {
  letter-spacing: 0.025em;
}

.heading::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  width: 2px;
  height: 2px;
  border-radius: 50%;
  opacity: 1;
  box-shadow: 220px 118px #fff, 280px 176px #fff, 40px 50px #fff,
    60px 180px #fff, 120px 130px #fff, 180px 176px #fff, 220px 290px #fff,
    520px 250px #fff, 400px 220px #fff, 50px 350px #fff, 10px 230px #fff;
  z-index: -1;
  transition: 1s ease;
  animation: 1s glowing-stars linear alternate infinite;
  animation-delay: 0s;
}
.icons::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  width: 2px;
  height: 2px;
  border-radius: 50%;
  opacity: 1;
  box-shadow: 140px 20px #fff, 425px 20px #fff, 70px 120px #fff, 20px 130px #fff,
    110px 80px #fff, 280px 80px #fff, 250px 350px #fff, 280px 230px #fff,
    220px 190px #fff, 450px 100px #fff, 380px 80px #fff, 520px 50px #fff;
  z-index: -1;
  transition: 1.5s ease;
  animation: 1s glowing-stars linear alternate infinite;
  animation-delay: 0.4s;
}
.icons::after {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  width: 2px;
  height: 2px;
  border-radius: 50%;
  opacity: 1;
  box-shadow: 490px 330px #fff, 420px 300px #fff, 320px 280px #fff,
    380px 350px #fff, 546px 170px #fff, 420px 180px #fff, 370px 150px #fff,
    200px 250px #fff, 80px 20px #fff, 190px 50px #fff, 270px 20px #fff,
    120px 230px #fff, 350px -1px #fff, 150px 369px #fff;
  z-index: -1;
  transition: 2s ease;
  animation: 1s glowing-stars linear alternate infinite;
  animation-delay: 0.8s;
}
.card:hover .heading::before,
.card:hover .icons::before,
.card:hover .icons::after {
  filter: blur(3px);
}

.image:active {
  cursor: -webkit-grabbing;
  cursor: grabbing;
}
.image:active + .heading::before {
  box-shadow: 240px 20px #9b40fc, 240px 25px #9b40fc, 240px 30px #9b40fc,
    240px 35px #9b40fc, 240px 40px #9b40fc, 242px 45px #9b40fc,
    246px 48px #9b40fc, 251px 49px #9b40fc, 256px 48px #9b40fc,
    260px 45px #9b40fc, 262px 40px #9b40fc;
  animation: none;
  filter: blur(0);
  border-radius: 2px;
  width: 0.45em;
  height: 0.45em;
  scale: 0.65;
  transform: translateX(9em) translateY(1em);
}
.image:active ~ .icons::before {
  box-shadow: 262px 35px #9b40fc, 262px 30px #9b40fc, 262px 25px #9b40fc,
    262px 20px #9b40fc, 275px 20px #9b40fc, 275px 24px #9b40fc,
    275px 28px #9b40fc, 275px 32px #9b40fc, 275px 36px #9b40fc,
    275px 40px #9b40fc, 275px 44px #9b40fc, 275px 48px #9b40fc;
  animation: none;
  filter: blur(0);
  border-radius: 2px;
  width: 0.45em;
  height: 0.45em;
  scale: 0.65;
  transform: translateX(9em) translateY(1em);
}
.image:active ~ .icons::after {
  box-shadow: 238px 60px #9b40fc, 242px 60px #9b40fc, 246px 60px #9b40fc,
    250px 60px #9b40fc, 254px 60px #9b40fc, 258px 60px #9b40fc,
    262px 60px #9b40fc, 266px 60px #9b40fc, 270px 60px #9b40fc,
    274px 60px #9b40fc, 278px 60px #9b40fc, 282px 60px #9b40fc,
    234px 60px #9b40fc, 234px 60px #9b40fc;
  animation: none;
  filter: blur(0);
  border-radius: 2px;
  width: 0.45em;
  height: 0.45em;
  scale: 0.65;
  transform: translateX(9em) translateY(1.25em);
}

.heading::after {
  content: "";
  top: -8.5%;
  left: -8.5%;
  position: absolute;
  width: 7.5em;
  height: 7.5em;
  border: none;
  outline: none;
  border-radius: 50%;
  background: #f9f9fb;
  box-shadow: 0px 0px 100px rgba(193, 119, 241, 0.8),
    0px 0px 100px rgba(135, 42, 211, 0.8), inset #9b40fc 0px 0px 40px -12px;
  transition: 0.4s ease-in-out;
  z-index: -1;
}
.card:hover .heading::after {
  box-shadow: 0px 0px 200px rgba(193, 119, 241, 1),
    0px 0px 200px rgba(135, 42, 211, 1), inset #9b40fc 0px 0px 40px -12px;
}

.icons {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
  column-gap: 1em;
  z-index: 1;
}

.instagram,
.x,
.discord {
  position: relative;
  transition: 0.4s ease-in-out;
}
.instagram:after,
.x:after,
.discord:after {
  content: "";
  position: absolute;
  width: 0.5em;
  height: 0.5em;
  left: 0;
  background-color: white;
  border-radius: 50%;
  z-index: -1;
  transition: 0.3s ease-in-out;
}
.instagram svg path,
.x svg path,
.discord svg path {
  stroke: #808080;
  transition: 0.4s ease-in-out;
}
.instagram:hover svg path {
  stroke: #cc39a4;
}
.x:hover svg path {
  stroke: black;
}
.discord:hover svg path {
  stroke: #8c9eff;
}
.instagram svg,
.x svg,
.discord svg {
  transition: 0.3s ease-in-out;
}
.instagram:hover svg {
  scale: 1.4;
}
.x:hover svg,
.discord:hover svg {
  scale: 1.25;
}
.instagram:hover:after,
.x:hover:after,
.discord:hover:after {
  scale: 4;
  transform: translateX(0.09em) translateY(0.09em);
}

.instagram::before {
  content: "";
  position: absolute;
  top: -700%;
  left: 1050%;
  rotate: -45deg;
  width: 5em;
  height: 1px;
  background: linear-gradient(90deg, #ffffff, transparent);
  animation: 4s shootingStar ease-in-out infinite;
  transition: 1s ease;
  animation-delay: 1s;
}
.x::before {
  content: "";
  position: absolute;
  top: -1300%;
  left: 850%;
  rotate: -45deg;
  width: 5em;
  height: 1px;
  background: linear-gradient(90deg, #ffffff, transparent);
  animation: 4s shootingStar ease-in-out infinite;
  animation-delay: 3s;
}
.discord::before {
  content: "";
  position: absolute;
  top: -2100%;
  left: 850%;
  rotate: -45deg;
  width: 5em;
  height: 1px;
  background: linear-gradient(90deg, #ffffff, transparent);
  animation: 4s shootingStar ease-in-out infinite;
  animation-delay: 5s;
}
.card:hover .instagram::before,
.card:hover .x::before,
.card:hover .discord::before {
  filter: blur(3px);
}
.image:active ~ .icons .instagram::before,
.image:active ~ .icons .x::before,
.image:active ~ .icons .discord::before {
  animation: none;
  opacity: 0;
}

@keyframes shootingStar {
  0% {
    transform: translateX(0) translateY(0);
    opacity: 1;
  }
  50% {
    transform: translateX(-55em) translateY(0);
    opacity: 1;
  }
  70% {
    transform: translateX(-70em) translateY(0);
    opacity: 0;
  }
  100% {
    transform: translateX(0) translateY(0);
    opacity: 0;
  }
}

@keyframes move {
  0% {
    transform: translateX(0em) translateY(0em);
  }
  25% {
    transform: translateY(-1em) translateX(-1em);
    rotate: -10deg;
  }
  50% {
    transform: translateY(1em) translateX(-1em);
  }
  75% {
    transform: translateY(-1.25em) translateX(1em);
    rotate: 10deg;
  }
  100% {
    transform: translateX(0em) translateY(0em);
  }
}

@keyframes glowing-stars {
  0% {
    opacity: 0;
  }

  50% {
    opacity: 1;
  }

  100% {
    opacity: 0;
  }
}

.dashboard-wrapper {
  min-height: calc(100vh - 100px); /* чтобы заняло почти весь экран */
  overflow-y: auto; /* прокрутка если контента много */
  border: 2px solid transparent;
  border-radius: 20px;
  background-clip: padding-box, border-box;
  background-origin: padding-box, border-box;
  background-image: linear-gradient(#171717, #171717), linear-gradient(45deg, #909091, #797979);
}

.dashboard-content {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1.5rem;
  margin-top: 2rem;
}




/* Красивый кастомный скроллбар */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(156, 163, 175, 0.4); /* сероватый */
  border-radius: 8px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(156, 163, 175, 0.7); /* подсветка при наведении */
}

/* Для Firefox */
* {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.4) transparent;
}

input[type="checkbox"].smooth-uncheck + .slider:before {
  transition: transform 0.3s ease;
}

input[type="checkbox"].smooth-uncheck:not(:checked) + .slider:before {
  transform: translateX(0px); /* плавно сдвигаем назад */
}

  .arrow {
    font-size: 16px;
    transition: transform 0.2s ease;
  }
  .arrow.down {
    transform: rotate(90deg);
  }

.switch {
  position: relative;
  display: inline-block;
  width: 34px;
  height: 18px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

/* Трек переключателя */
.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #2a2a2a; /* Темный фон */
  border: 1px solid #3a3a3a; /* Едва заметная рамка */
  border-radius: 9999px;
  transition: background-color 0.2s, border-color 0.2s;
}

/* Ползунок */
.slider:before {
  position: absolute;
  content: "";
  height: 14px;
  width: 14px;
  left: 2px;
  top: 1px;
  background-color: #d1d1d1; /* Светло-серый ползунок */
  border-radius: 50%;
  transition: transform 0.2s;
}

/* Состояние "включено" */
input:checked + .slider {
  background-color: #535353;
  border-color: #4a4a4a;
}

input:checked + .slider:before {
  transform: translateX(16px);
}


  .slider .on {
    display: none;
  }

  input:checked + .slider .on {
    display: block;
  }

  input:checked + .slider .off {
    display: none;
  }

  .slider .off {
    width: 100%;
    text-align: right;
  }
 .modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(15, 15, 15, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 50;
  animation: fadeIn 0.3s ease-in-out;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.modal-content {
  background: #171717;
  border-radius: 20px;
  padding: 30px;
  width: 90%;
  max-width: 450px;
  box-shadow: inset 0 0 10px #0a0a0a, 0 0 20px rgba(0,0,0,0.5);
  position: relative;
  color: white;
  animation: slideUp 0.4s ease-out;
}

@keyframes slideUp {
  from { transform: translateY(30px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

.modal-title {
  font-size: 1.5rem;
  font-weight: bold;
  margin-bottom: 1.5rem;
  text-align: center;
}

.modal-close {
  position: absolute;
  top: 15px;
  right: 20px;
  font-size: 1.5rem;
  color: #888;
  background: none;
  border: none;
  cursor: pointer;
  transition: color 0.3s;
}

.modal-close:hover {
  color: #fff;
}

.modal-input {
  width: 100%;
  background: #2e2e2e;
  border: none;
  border-radius: 12px;
  padding: 12px 16px;
  color: #d3d3d3;
  outline: none;
  transition: background 0.3s;
}

.modal-input::placeholder {
  color: #777;
}

.modal-input:hover {
  background: #3a3a3a;
}

.modal-button {
  width: 100%;
  margin-top: 1rem;
  background-color: #252525;
  padding: 12px;
  border-radius: 12px;
  color: white;
  font-weight: bold;
  text-align: center;
  transition: background-color 0.3s;
}

.modal-button:hover {
  background-color: black;
}

.modal-upload-button {
  width: 100%;
  margin-top: 0.5rem;
  background-color: #2e2e2e;
  padding: 12px;
  border-radius: 12px;
  color: #d3d3d3;
  font-size: 0.9rem;
  text-align: center;
  cursor: pointer;
  transition: background-color 0.3s;
}

.modal-upload-button:hover {
  background-color: #3a3a3a;
}

.modal-checkbox {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
  color: #d3d3d3;
  margin-top: 0.5rem;
}


/* Анимации */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes scaleIn {
  from { transform: scale(0.9); opacity: 0; }
  to { transform: scale(1); opacity: 1; }
}

.collapsible-content {
  overflow: hidden;
  max-height: 0;
  transition: max-height 0.3s ease, opacity 0.3s ease, transform 0.3s ease;
  opacity: 0;
  transform: translateY(-10px);
}

.collapsible-content.open {
  padding-top: 6px;
  max-height: 1000px;
  opacity: 1;
  transform: translateY(0);
}


.company-title {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #d3d3d3;
  font-weight: 500;
  font-size: 0.9rem;
  cursor: pointer;
  transition: color 0.2s;
}

.company-title:hover {
  color: #4b9eff;
}


.company-title.active {
  color: #4b9eff;
  border-bottom-color: #4b9eff;
}


.company-block {
  background-color: #181818;

  border-radius: 12px;
  padding: 10px 4px;
  margin-bottom: 3px;
  transition: background-color 0.2s ease, border-color 0.2s ease;
}

/* Стили для настроек */
.settings-container {
  padding: 20px;
  color: white;
}

.settings-section {
  background: #1f1f1f;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
}

.settings-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #e0e0e0;
  margin-bottom: 15px;
}

.settings-group {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.settings-label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: #d3d3d3;
}

.settings-select {
  background: #2e2e2e;
  border: 1px solid #3a3a3a;
  border-radius: 8px;
  padding: 8px 12px;
  color: #e0e0e0;
  min-width: 150px;
}

.settings-actions {
  display: flex;
  gap: 10px;
  margin-top: 20px;
}

.settings-button {
  padding: 10px 20px;
  border-radius: 8px;
  font-weight: 500;
  transition: all 0.2s;
}

.settings-button.save {
  background: #4b9eff;
  color: white;
}

.settings-button.save:hover {
  background: #3a8eff;
}

.settings-button.reset {
  background: #2e2e2e;
  color: #e0e0e0;
}

.settings-button.reset:hover {
  background: #3a3a3a;
}

/* Стили для раздела активности */
.activity-container {
  padding: 20px;
  color: white;
}

.activity-section {
  background: #1f1f1f;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
}

.activity-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #e0e0e0;
  margin-bottom: 15px;
}

/* Стили для статистики */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
}

.stat-card {
  background: #2a2a2a;
  border-radius: 10px;
  padding: 15px;
  display: flex;
  align-items: center;
  gap: 15px;
}

.stat-icon {
  width: 40px;
  height: 40px;
  background: #3a3a3a;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2rem;
  color: #4b9eff;
}

.stat-info {
  display: flex;
  flex-direction: column;
}

.stat-value {
  font-size: 1.2rem;
  font-weight: 600;
  color: #fff;
}

.stat-label {
  font-size: 0.8rem;
  color: #a0a0a0;
}

/* Стили для графика */
.chart-container {
  background: #2a2a2a;
  border-radius: 10px;
  padding: 20px;
  height: 200px;
}

.chart-placeholder {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  height: 100%;
  padding: 20px 0;
}

.chart-bar {
  width: 30px;
  background: #4b9eff;
  border-radius: 4px 4px 0 0;
  transition: height 0.3s ease;
}

/* Стили для истории */
.history-filters {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  align-items: center;
}

.history-select,
.history-search {
  background: #2a2a2a;
  border: 1px solid #3a3a3a;
  border-radius: 8px;
  padding: 8px 12px;
  color: #e0e0e0;
}

.history-search {
  flex: 1;
}

.history-table {
  background: #2a2a2a;
  border-radius: 10px;
  overflow: hidden;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  padding: 12px 15px;
  text-align: left;
  border-bottom: 1px solid #3a3a3a;
}

th {
  background: #1f1f1f;
  color: #a0a0a0;
  font-weight: 500;
  font-size: 0.9rem;
}

td {
  color: #e0e0e0;
  font-size: 0.9rem;
}

.status-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.8rem;
  text-transform: capitalize;
}

.status-badge.success {
  background: rgba(0, 206, 62, 0.2);
  color: #00ce3e;
}

.status-badge.error {
  background: rgba(255, 59, 48, 0.2);
  color: #ff3b30;
}

.status-badge.pending {
  background: rgba(255, 204, 0, 0.2);
  color: #ffcc00;
}

/* Стили для раздела поддержки */
.support-container {
  padding: 20px;
  color: white;
}

.support-section {
  background: #1f1f1f;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
}

.support-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #e0e0e0;
  margin-bottom: 20px;
}

/* Стили для формы обратной связи */
.feedback-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-group label {
  color: #a0a0a0;
  font-size: 0.9rem;
}

.form-select,
.form-input,
.form-textarea {
  background: #2a2a2a;
  border: 1px solid #3a3a3a;
  border-radius: 8px;
  padding: 10px 12px;
  color: #e0e0e0;
  font-size: 0.9rem;
}

.form-textarea {
  resize: vertical;
  min-height: 100px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #a0a0a0;
  font-size: 0.9rem;
}

.submit-button {
  background: #4b9eff;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.submit-button:hover {
  background: #3a8eff;
}

/* Стили для FAQ */
.faq-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.faq-item {
  background: #2a2a2a;
  border-radius: 8px;
  overflow: hidden;
}

.faq-question {
  width: 100%;
  padding: 15px;
  background: none;
  border: none;
  color: #e0e0e0;
  text-align: left;
  font-size: 0.95rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  transition: background-color 0.2s;
}

.faq-question:hover {
  background: #3a3a3a;
}

.faq-question.active {
  background: #3a3a3a;
}

.faq-question i {
  transition: transform 0.2s;
}

.faq-question.active i {
  transform: rotate(180deg);
}

.faq-answer {
  padding: 15px;
  color: #a0a0a0;
  font-size: 0.9rem;
  line-height: 1.5;
  background: #2a2a2a;
  border-top: 1px solid #3a3a3a;
}

/* FAQ Category Styles */
.faq-category {
  margin-bottom: 20px;
}

.faq-category-title {
  color: #e0e0e0;
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 15px;
  padding-bottom: 8px;
  border-bottom: 1px solid #3a3a3a;
}

.faq-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.faq-item {
  background: #2a2a2a;
  border-radius: 8px;
  overflow: hidden;
}

.faq-question {
  width: 100%;
  padding: 15px;
  background: none;
  border: none;
  color: #e0e0e0;
  text-align: left;
  font-size: 0.95rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  transition: background-color 0.2s;
}

.faq-question:hover {
  background: #3a3a3a;
}

.faq-question.active {
  background: #3a3a3a;
}

.faq-question i {
  transition: transform 0.2s;
}

.faq-question.active i {
  transform: rotate(180deg);
}

.faq-answer {
  padding: 15px;
  color: #a0a0a0;
  font-size: 0.9rem;
  line-height: 1.5;
  background: #2a2a2a;
  border-top: 1px solid #3a3a3a;
}

.history-filters {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  align-items: center;
}

.refresh-button {
  background: none;
  border: none;
  color: #fff;
  cursor: pointer;
  padding: 8px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.refresh-button:hover {
  background: rgba(255, 255, 255, 0.1);
}

.refresh-button:active {
  background: rgba(255, 255, 255, 0.2);
}

.refresh-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.fa-spin {
  animation: fa-spin 1s infinite linear;
}

@keyframes fa-spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>