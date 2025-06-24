<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte';
    import { user, serverdata, activeTab } from '../store';
    import type { User, ScreenSize, Server, Service, ServerInfo, MainInfo,ServerResponse, ServiceStatus,  ServiceInfo, ConnectRDPParams } from '../types';
    
    
  
    let isEnabled = false;
  
    function handleSwitchChange(value: boolean) {
      isEnabled = value;
      console.log('Switch is now', isEnabled ? 'on' : 'off');
    }

    let companyInfo:ServerResponse[] = [];
    let filteredCompanyInfo: ServerResponse[] = [];
  
    let serverInfo: ServerInfo[] = [];
    let mainInfo: MainInfo | null = null;
    let expandedCompanyIds: string[] = [];
    let searchQuery = '';
    
  
    async function fetchMainInfo() {
        mainInfo = await invoke('get_servers');
        companyInfo = mainInfo.companys;
        console.log(mainInfo)
        filteredCompanyInfo = companyInfo;
    }

    

    function getServiceInfo(serviceId: string): ServiceStatus {
        return mainInfo.service_status[serviceId];
    };

    function getServiceStatusById(serviceId: string): boolean  {
        const serviceStatus = mainInfo.service_status[serviceId];
        if (serviceStatus) {
            return serviceStatus.is_connected;
        }
        return false;
    }

    function toggleCompany(companyId: string) {
        if (expandedCompanyIds.includes(companyId)) {
            expandedCompanyIds = expandedCompanyIds.filter(id => id !== companyId);
        } else {
            expandedCompanyIds = [...expandedCompanyIds, companyId];
        }
    }
  
    async function toggleConnection(tunnelid: string, protocol: string, isstarted: boolean) {
        if (protocol === 'rdp') {
            await invoke('set_connect_rdp', { tunnelid, isstarted });
        } else if (protocol === 'ssh') {
            await invoke('set_connect_ssh', { tunnelid, isstarted });
        }

        await fetchMainInfo(); // Refresh server info after toggling
    }

  
    function handleCheckboxChange(event: Event, server: Service, companyid:string) {
        const checkbox = event.target as HTMLInputElement; 
        let isstarted = checkbox.checked;
        toggleConnection(server.id, server.protocol, isstarted);
        if (isstarted) {
            handleServerLogin(server, companyid)
        }
    }

    function handleServerLogin(server: Service, companyid: string) {
        let serviceState = getServiceInfo(server.id)
        let serviceInfo: ServiceInfo = {
            service: server, // Исправлено название поля
            status: serviceState,
            companyid: companyid
        };
            
        serverdata.set(serviceInfo); // Записываем текущий сервер в server_data
        activeTab.set('loginserver'); // Переключаем вкладку на loginserver
    }
  
    
  
    function handleMaxScreenChange(event: Event) {
        const checkbox = event.target as HTMLInputElement;
        let isMaxScreen = checkbox.checked;
        if (isMaxScreen) {
            // Здесь мы можем очистить или сделать что-то еще с размерами экрана
        }
    }
  
    async function updateSettings(tunnelid: string) {
        const server = serverInfo.find(info => info.server.tunnel_id === tunnelid);
        if (server) {
            server.rdp_param.screen_size.width = (document.getElementById('width') as HTMLInputElement).valueAsNumber;
            server.rdp_param.screen_size.height = (document.getElementById('height') as HTMLInputElement).valueAsNumber;
  
            await invoke('update_server_connect_params', {
                tunnelid: server.rdp_param.tunnel_id,
                issound: server.rdp_param.is_sound,
                isdrives: server.rdp_param.is_drives,
                isprinters: server.rdp_param.is_printers,
                iswallpaper: server.rdp_param.is_wallpaper,
                ismaxscreen: server.rdp_param.is_max_screen,
                w: server.rdp_param.screen_size.width,
                h: server.rdp_param.screen_size.height,
            });
            await fetchServerInfo(); // Обновление информации о серверах после изменений
        }
    }


    function handleSvgClick(localport: number) {
      const textToCopy = `localhost:${localport}`;
      navigator.clipboard.writeText(textToCopy).then(() => {
          showNotification('Скопировано!');
      }).catch(err => {
          console.error('Failed to copy text: ', err);
      });
    }
    console.log(user)
    function filteredCompanies() {
        
        if (!searchQuery) {
            return companyInfo; // Если поисковый запрос пуст, возвращаем весь список
        }
        return companyInfo.filter(company => {
            const isCompanyMatch = company.name.toLowerCase().includes(searchQuery.toLowerCase());
            const isServerMatch = company.servers.some(server => 
                server.name.toLowerCase().includes(searchQuery.toLowerCase())
            );
            return isCompanyMatch || isServerMatch;
        });
        
    }


    $: filteredCompanyInfo = companyInfo.filter(company => {
        const isCompanyMatch = company.name.toLowerCase().includes(searchQuery.toLowerCase());
        const isServerMatch = company.servers.some(server => 
            server.name.toLowerCase().includes(searchQuery.toLowerCase())
        );
        return isCompanyMatch || isServerMatch;
    }); 

  
  
onMount(() => {
    fetchMainInfo();
        //fetchServerInfo(); // Выполняем первый вызов при монтировании компонента
        //const interval = setInterval(fetchMainInfo, 15000); // Устанавливаем интервал на 2 секунды
        
        // Возвращаем функцию очистки, которая вызовется при размонтировании компонента
        //return () => clearInterval(interval);
    });
</script>
  


<div class="base-server">
    <div class="notification" id="notification">Скопировано!</div>
    <div class="search-container">
        <input
            type="text"
            placeholder="Поиск по имени компании или сервера..."
            bind:value={searchQuery} />
    </div>

    {#if user != null}
        <div class="accordion">
            
            {#if mainInfo != null && filteredCompanyInfo.length > 0}
            
{#each filteredCompanyInfo as company (company.id)}
  <div>
    <div class="company-header" on:click={() => toggleCompany(company.id)}>
      <span class="arrow {expandedCompanyIds.includes(company.id) ? 'down' : ''}">▶</span>
      <span class="company-name">{company.name}</span>
    </div>

    {#if expandedCompanyIds.includes(company.id)}
      <div class="server-list">
        {#each company.servers as serv (serv.id)}
          {#each serv.services as servise (servise.id)}
            <div class="service-block">
              <div class="service-info">
                <h5>СЕРВЕР: {serv.name}</h5>
                <div class="protocol">{servise.protocol}</div>
              </div>

              <label class="switch">
                <input
                  type="checkbox"
                  id="flexSwitchCheckDefault-{servise.id}"
                  checked={getServiceStatusById(servise.id)}
                  on:change={(e) => handleCheckboxChange(e, servise, company.id)}
                />
                <span class="slider">
                  <span class="on">ON</span>
                  <span class="off">OFF</span>
                </span>
              </label>
            </div>
          {/each}
        {/each}
      </div>
    {/if}
  </div>
{/each}

            {/if}
        </div>
    {:else}
        <p>Этот раздел требует авторизации</p>
    {/if}
</div>



  
  <style>


.search-container {
    margin: 0 5px;
    margin-bottom: 20px;
}

.search-container input {
    
    width: 100%;
    padding: 10px;
    font-size: 16px;
    border: 0px solid #000000;
    border-radius: 5px;
    background: rgba(82, 81, 81, 0.5);

    
}

.search-container input:focus {
    outline: none;
    color: #b8b8b8;
    border-color: #000000; /* если нужно изменить цвет рамки */
}

.base-server{
    max-height: 400px; /* Вы можете изменить это значение в зависимости от вашего дизайна */
  
  /* Разрешаем появление полосы прокрутки, если содержимое превышает высоту контейнера */
  overflow-y: auto;
  scrollbar-width: thin; /* Или 'auto' для стандартной ширины */
    scrollbar-color: rgba(0, 188, 212, 1) rgb(184, 184, 184);
}
.custom-button2 {
    background-image: url('/path/to/button.png');
    background-size: contain; 
    background-repeat: no-repeat;
    background-position: center;
    width: 50px; 
    height: 50px; 
    border: none;
    margin-right: 5px;
    cursor: pointer; 
    background-color: transparent; 
}
.custom-button2:hover {
    transform: scale(1.03); /* Легкое увеличение при наведении */
}
  .clickable-svg {
    cursor: pointer; /* Указатель при наведении на SVG */
    transition: fill 0.2s ease; /* Плавный переход цвета при наведении */
  }
  

  .clickable-svg:hover {
    fill: #ccc; /* Цвет заливки при наведении на SVG */
  }
  .company-name{
    padding-left: 20px;
  }
  .server-info{
    padding-left: 50px;
  }
  .custom-table {
  color: rgb(221, 215, 215);
  height: 80px;
  width: 100%;
  
  border-bottom: 0.5px solid rgb(115, 115, 115); /* Добавление черты внизу каждой строки */
  }

  .custom-line {
    padding: 10px 0; /* Добавляем отступы для центрирования содержимого по вертикали */
  }

  .switch {
      position: relative;
      display: inline-block;
      width: 55px;
      height: 30px;
  }

  .switch input {
    margin-left: auto;
      opacity: 0;
      width: 0;
      height: 0;
  }

  .slider {
      position: absolute;
      cursor: pointer;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background-color: #ccc;
      transition: .4s;
      border-radius: 30px;
      display: flex;
      align-items: center;
      justify-content: space-between; /* Равномерно распределяет текст */
      padding: 0 7px;
      font-size: 11px;
      color: rgb(5, 5, 5);
      font-weight: bold;
  }

  .slider:before {
      position: absolute;
      content: "";
      height: 23px;
      width: 23px;
      left: 2px;
      bottom: 4px;
      background-color:  rgb(249, 246, 246);
      transition: .4s;
      border-radius: 50%;
  }

  input:checked + .slider {
      background-color: rgba(0, 188, 212, 1);

  }

  input:checked + .slider:before {
      transform: translateX(26px);
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
      
      text-align: right; /* Прибиваем текст к правому краю */
      width: 100%;
  }





  .company-header {
        cursor: pointer;
        
        padding: 10px;
        margin-bottom: 5px;
        font-weight: bold;
        border-radius: 4px;
        display: flex;
        justify-content: flex-start;
        align-items: center;
    }

    .server-list {
        margin-bottom: 10px;
        padding: 10px;
        border-radius: 4px;
    }

    .notification {
        display: none;
        position: absolute;
        top: 10px;
        right: 10px;
        padding: 10px;
        background-color: #4caf50;
        color: white;
        border-radius: 5px;
    }

    .active {
        display: block;
    }

    .arrow {
    margin-right: 0px;
    font-size: 16px;
    background: radial-gradient(94.3% 185.81% at 94.3% 50%, #6868DD 0%, #5773DB 52%, #7B5CE0 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
    }

    .arrow.down {
        transform: rotate(90deg);
    }
    .company-name {
    font-size: 18px;
    background: radial-gradient(94.3% 185.81% at 94.3% 50%, #6868DD 0%, #5773DB 52%, #7B5CE0 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-fill-color: transparent;
    } 

.service-block {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border: 2px solid #6868DD; /* Цвет рамки */
    padding: 10px;
    color: #6868DD; /* Цвет текста */
    font-size: 16px;
    border-radius: 5px;
    background: none; /* Без фона */
}


.server-list {
    margin-bottom: 10px;
    padding: 10px;
    border-radius: 4px;
}

.service-block {
    border: 1px solid #5e33d1; /* Граница блока */
    border-radius: 8px;
    padding: 10px;
    margin-bottom: 15px;
}

.service-row {
    width: 100%;
    display: flex;
    justify-content: space-between; /* Распределение по краям */
    align-items: center; /* Выравнивание по вертикали */
}

h5 {
    margin: 0;
    font-size: 18px;
    color: #fff; /* Белый цвет текста */
}

.protocol {
    margin-top: 5px;
    font-size: 14px;
    color: #a0a0a0; /* Светло-серый цвет для протокола */
}







</style>


