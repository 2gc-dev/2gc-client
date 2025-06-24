<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from 'svelte-i18n';
  import { user } from '../store';  // �� Импорт user store
  import { USER_AGENT } from './constants';
  import { platform } from '@tauri-apps/plugin-os';
  import { getVersion } from '@tauri-apps/api/app';

  let apiBaseUrl = "https://app.chatwoot.com/public/api/v1";
  let inboxIdentifier = "Dh3EVc4EiZbFE9C3FA6Sag29";
  let sseBaseUrl = "https://chat-realtime-server.2gc.app/events";

  let contactIdentifier = localStorage.getItem('contactIdentifier') || crypto.randomUUID();
  localStorage.setItem('contactIdentifier', contactIdentifier);

  let conversationId = localStorage.getItem('conversationId');
  let eventSource: EventSource | null = null;
  let messageInput = '';
  let messages: { author: string; content: string; isUser: boolean }[] = [];

  let messagesEnd: HTMLDivElement | null = null;

  let clientInfo = {
    version: '',
    os: '',
    arch: '',
    screen_resolution: '',
    display_count: 0,
    timezone: '',
    locale: '',
    color_depth: 0,
    device_memory: 0,
    hardware_concurrency: 0,
    language: '',
    platform: '',
    vendor: '',
    user_agent: ''
  };

  function scrollToBottom() {
    messagesEnd?.scrollIntoView({ behavior: 'smooth' });
  }

  $: messages, scrollToBottom();

  function appendMessage(author: string, content: string, isUser = false) {
    messages = [...messages, { author, content, isUser }];
  }

  async function sendIncomingMessage() {
    const msg = messageInput.trim();
    if (!msg) return;

    appendMessage($t('you_label'), msg, true);

    try {
      // 👇 Получение имени пользователя из store
      const contactName = $user?.name || 'Гость';

      const contactRes = await fetch(`${apiBaseUrl}/inboxes/${inboxIdentifier}/contacts`, {
        method: 'POST',
        headers: { 
          'Content-Type': 'application/json',
          'User-Agent': USER_AGENT,
          'X-Client-Type': 'desktop',
          'X-Client-Version': clientInfo.version,
          'X-Client-OS': clientInfo.os,
          'X-Client-Arch': clientInfo.arch,
          'X-Client-Screen': clientInfo.screen_resolution,
          'X-Client-Displays': clientInfo.display_count.toString(),
          'X-Client-Timezone': clientInfo.timezone,
          'X-Client-Locale': clientInfo.locale,
          'X-Client-ColorDepth': clientInfo.color_depth.toString(),
          'X-Client-Memory': clientInfo.device_memory.toString(),
          'X-Client-Cores': clientInfo.hardware_concurrency.toString(),
          'X-Client-Language': clientInfo.language,
          'X-Client-Platform': clientInfo.platform,
          'X-Client-Vendor': clientInfo.vendor
        },
        body: JSON.stringify({
          identifier: contactIdentifier,
          name: $user?.name || 'Гость',
          email: $user?.email || '',
          custom_attributes: {
            company: $user?.company || 'Не указано'
          }
        })
      });

      const contactData = await contactRes.json();
      const contactSourceId = contactData.source_id;

      let convId = conversationId;
      if (!convId) {
        const convRes = await fetch(`${apiBaseUrl}/inboxes/${inboxIdentifier}/contacts/${contactSourceId}/conversations`, {
          method: 'POST',
          headers: { 
            'Content-Type': 'application/json',
            'User-Agent': USER_AGENT,
            'X-Client-Type': 'desktop',
            'X-Client-Version': clientInfo.version,
            'X-Client-OS': clientInfo.os,
            'X-Client-Arch': clientInfo.arch
          },
          body: JSON.stringify({ custom_attributes: {} })
        });
        const convData = await convRes.json();
        convId = convData.id;
        conversationId = convId;
        localStorage.setItem('conversationId', conversationId);
        reconnectSSE();
      }

      await fetch(`${apiBaseUrl}/inboxes/${inboxIdentifier}/contacts/${contactSourceId}/conversations/${convId}/messages`, {
        method: 'POST',
        headers: { 
          'Content-Type': 'application/json',
          'User-Agent': USER_AGENT,
          'X-Client-Type': 'desktop',
          'X-Client-Version': clientInfo.version,
          'X-Client-OS': clientInfo.os,
          'X-Client-Arch': clientInfo.arch
        },
        body: JSON.stringify({ content: msg })
      });

    } catch (error) {
      console.error('Public API send error:', error);
    }

    messageInput = '';
  }

  function setupSSE() {
    if (eventSource) eventSource.close();

    const sseUrl = conversationId
      ? `${sseBaseUrl}?conversation_id=${conversationId}`
      : sseBaseUrl;

    eventSource = new EventSource(sseUrl);

    eventSource.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        console.log('SSE Event:', data);

        if (data.type !== 'message_created') return;
        if (data.sender_id && data.sender_id === contactIdentifier) return;
        if (data.sender_type && data.sender_type !== 'Agent') return;

        appendMessage(data.sender, data.message, false);
      } catch (e) {
        console.error('SSE processing error:', e);
      }
    };

    eventSource.onerror = (error) => {
      console.error('SSE connection error:', error);
    };
  }

  function reconnectSSE() {
    setupSSE();
  }

  onMount(async () => {
    // Получаем информацию о дисплеях
    const displays = window.screen.width + 'x' + window.screen.height;
    const displayCount = window.screen.availWidth > 0 ? 1 : 0; // Простая проверка на наличие дисплея

    clientInfo = {
      version: await getVersion(),
      os: await platform(),
      arch: navigator.platform,
      screen_resolution: displays,
      display_count: displayCount,
      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
      locale: navigator.language,
      color_depth: window.screen.colorDepth,
      device_memory: (navigator as any).deviceMemory || 0,
      hardware_concurrency: navigator.hardwareConcurrency || 0,
      language: navigator.language,
      platform: navigator.platform,
      vendor: navigator.vendor,
      user_agent: navigator.userAgent
    };
    setupSSE();
  });
</script>


<div class="chat-widget rounded-lg shadow-lg  text-white border border-gray-700 max-w-md mx-auto flex flex-col h-[400px]">
  <div class="messages flex-1 overflow-y-auto p-3 space-y-2">
    {#each messages as m}
      <div class="flex {m.isUser ? 'justify-end' : 'justify-start'}">
        <div class="{m.isUser ? 'bg-green-600' : 'bg-blue-600'} rounded-xl px-3 py-2 max-w-[80%] break-words">
          <span class="text-xs text-gray-300 block">{m.author}</span>
          <span class="text-sm">{m.content}</span>
        </div>
      </div>
    {/each}
    <div bind:this={messagesEnd}></div>
  </div>

  <div class="flex p-2 border-t border-gray-700">
    <input
      type="text"
      class="flex-1 p-2 bg-gray-800 rounded-l-md text-white outline-none focus:ring-2 focus:ring-blue-500"
      bind:value={messageInput}
      on:keydown={(e) => { if (e.key === 'Enter') sendIncomingMessage(); }}
      placeholder="{$t('type_message_placeholder')}"
    />
    <button 
      on:click={sendIncomingMessage}
      class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-r-md transition-colors duration-200"
    >
      {$t('send_button')}
    </button>
  </div>
</div>