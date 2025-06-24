<script lang="ts">


  import { invoke } from "@tauri-apps/api/core";
  import { user } from '../store';
  import { onMount } from 'svelte';
  import Heder from '../lib/Heder.svelte';
  import Auth from '../lib/Auth.svelte';
  import Server from '../lib/Server.svelte';
  import type { User, ServerInfo } from '../types';
  import { activeTab } from '../store';
  import ConfigServer from '../lib/ConfigServer.svelte';
  import Message from '../lib/Message.svelte';
  import LoginServer from '../lib/LoginServer.svelte';
  import { setupI18n } from '../i18n';
  import { Toaster } from 'svelte-french-toast';



  let i18nReady = false;
  let is_auth = false;
  let loading = true;

  function showAuth() {
    activeTab.set('auth');
  }

  function showServer() {
    activeTab.set('server');
  }

  function showMessage() {
    activeTab.set('message');
  }

  let server_data: ServerInfo = null;

  function changeserver(serverdata: ServerInfo | null) {
    server_data = serverdata;
  }

  async function try_remember() {
    try {
      if (localStorage.getItem('logout_flag') !== 'true') {
        const success = await invoke('try_remember_token');
        if (success) {
          const answer: User = await invoke('get_user');
          if (answer != null) {
            user.set(answer);
            activeTab.set('server');
          } else {
            activeTab.set('auth');
          }
        } else {
          activeTab.set('auth');
        }
      } else {
        activeTab.set('auth');
      }
    } catch (error) {
      console.error("Ошибка восстановления сессии:", error);
      activeTab.set('auth');
    } finally {
      loading = false;
    }
  }

  onMount(try_remember);
    onMount(async () => {
    await setupI18n(); // ждем инициализацию локали!
    i18nReady = true;
    await try_remember();
  });
</script>

<Heder/>
<Toaster />

<div class="custom-base">
  {#if !i18nReady}
    <div class="loading-message">Loading locale...</div>
  {:else if loading}
    <div class="loading-message">Загрузка данных, пожалуйста подождите...</div>
  {:else}
    {#if $activeTab === 'auth'}
      <Auth />
    {:else if $activeTab === 'server'}
      <Server />
    {:else if $activeTab === 'loginserver'}
      <LoginServer/>
    {:else if $activeTab === 'configserver'}
      <ConfigServer/>
    {:else if $activeTab === 'message'}
      <Message/>
    {/if}
  {/if}
</div>


<style>


html, body {
  height: 100%;
  width: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #171717;
}

.loading-message {
  color: #888;
  text-align: center;
  margin-top: 2rem;
  font-size: 1rem;
}

.custom-base {
  margin: 0 auto;
  overflow-y: auto;
  background-color: #171717;
  border-radius: 20px;
}

</style>
