<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { User } from '../types';
  import { user, activeTab } from '../store';
  import { t } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { tick } from 'svelte';


  let email = '';
  let password = '';
  let remember = false;
  let isLoading = false;
  let visible = false;
  let message = '';
  let loginErrorMessage = '';
  let hasLoginError = false;
  let showLoaderCard = false;


  $: user_state = $user;

  onMount(async () => {
    const remembered = await invoke<boolean>('try_remember_token');
    if (remembered) {
      const answer: User = await invoke('get_user');
      user.set(answer);
      activeTab.set('server');
    }
  });

  function showNotification(msg: string) {
    message = msg;
    visible = true;
    setTimeout(() => {
      visible = false;
    }, 1200);
  }

async function authUser(event: Event) {
  event.preventDefault();
  isLoading = true;
  hasLoginError = false;
  loginErrorMessage = '';

  try {
    const result: boolean = await invoke('auth_user', { email, password, remember });

    if (result) {
      showLoaderCard = true;

      await tick(); // подождать рендер
      await new Promise(resolve => setTimeout(resolve, 1000));

      const answer: User = await invoke('get_user');
      user.set(answer);

      await new Promise(resolve => setTimeout(resolve, 3000));
      showLoaderCard = false;
      activeTab.set('server');
    } else {
      hasLoginError = true;
      loginErrorMessage = $t('invalid_credentials');
    }
  } catch (error) {
    console.error("Ошибка авторизации:", error);
    hasLoginError = true;
    loginErrorMessage = String(error);
  } finally {
    isLoading = false;
  }
}
  async function signOut(event: Event) {
    await invoke('singout');
    await invoke('clear_storage');
    user.set(null);
    localStorage.setItem('logout_flag', 'true');
    showNotification($t('logged_out'));
  }

  async function clearFile(event: Event) {
    await invoke('clear_storage');
    showNotification($t('memory_cleared'));
  }

  function get_user_state(): boolean {
    return user_state === null;
  }
</script>




<section class="auth-form">
  {#if visible}
    <div class="notification">{message}</div>
  {/if}

  <div class="auth-card">
    {#if showLoaderCard}
      <!-- Показываем карточку с лоадером -->
      <div class="card">
        <div class="loader">
          <p>loading</p>
          <div class="words">
            <span class="word">buttons</span>
            <span class="word">forms</span>
            <span class="word">switches</span>
            <span class="word">cards</span>
            <span class="word">buttons</span>
          </div>
        </div>
      </div>
    {:else if $user === null}
      {#if isLoading}
        <div class="flex justify-center py-10">
          <div class="loader"></div>
        </div>
      {:else}
        <form class="space-y-6" on:submit|preventDefault={authUser}>
          <h2 class="auth-title">{$t('login_title')}</h2>

          <div class="space-y-4">
            <input
              type="text"
              placeholder={$t('email_placeholder')}
              class="auth-input"
              bind:value={email}
            />

            <input
              type="password"
              placeholder={$t('password_placeholder')}
              class="auth-input"
              bind:value={password}
            />

            <div class="auth-checkbox">
              <input type="checkbox" id="remember" bind:checked={remember} />
              <label for="remember">{$t('remember_me')}</label>
            </div>

            <button type="submit" class="auth-button">
              {$t('login_button')}
            </button>

            {#if hasLoginError}
              <div class="error-message mt-2 text-red-500 text-sm">
                {loginErrorMessage}
              </div>
            {/if}

            <div class="auth-links mt-4">
              <a href="http://localhost:5021/accounts/register/" target="_blank" rel="noopener noreferrer">
                {$t('register_link')}
              </a>
              <a href="http://localhost:5021/user/forgot-password/no-login/" target="_blank" rel="noopener noreferrer">
                {$t('forgot_password_link')}
              </a>
            </div>
          </div>
          <ul class="example-2">
      <div class="filled"></div>
  <li class="icon-content">
    <a
      data-social="dribbble"
      aria-label="Dribbble"
      href="https://2gc.io/"
      target="_blank"
    >
      <div class="filled"></div>
      <svg viewBox="0 0 100 100" version="1.1">
        <path
          fill="currentColor"
          d="M83.5,18.5C74.9,9.3,62.8,4,50.2,4c-6.1,0-12,1.1-17.6,3.4C15.2,14.5,4,31.3,4,50c0,13.9,6.2,26.9,17,35.7  C29.2,92.3,39.4,96,50,96c6.6,0,13.2-1.5,19.2-4.2c12.5-5.7,21.7-16.6,25.2-29.8C95.5,57.9,96,53.8,96,50  C96,38.3,91.6,27.1,83.5,18.5z M75,22.3c-0.7,0.9-1.4,1.8-2.1,2.6c-1.4,1.6-2.8,3-4.4,4.3c-0.3,0.3-0.6,0.6-1,0.8  c-1,0.9-2.1,1.7-3.2,2.5l-0.3,0.2c-1.1,0.7-2.2,1.5-3.5,2.2c-0.4,0.3-0.9,0.5-1.4,0.8c-0.8,0.5-1.7,0.9-2.7,1.4  c-0.6,0.3-1.2,0.5-1.8,0.8L54.3,38c-0.1,0-0.2,0.1-0.3,0.1c0,0,0,0,0,0c-1.3-2.6-2.4-4.9-3.5-7l-0.3-0.5c-1.1-2-2.2-4-3.3-6  l-0.7-1.3c-1.1-1.9-2.2-3.7-3.2-5.4l-0.7-1.1c-0.7-1.2-1.4-2.3-2.2-3.5c3.2-0.8,6.5-1.3,9.8-1.3c9.4,0,18.4,3.5,25.4,9.8  C75.3,21.9,75.2,22.1,75,22.3z M46.4,40.6c-1.4,0.4-2.9,0.8-4.4,1.1c-0.3,0-0.7,0.1-0.9,0.2c-6,1-12.5,1.4-19.4,1.1  c-0.3,0-0.6,0-0.9,0c-0.3,0-0.5,0-0.7,0c-2.5-0.2-4.9-0.4-7.2-0.7c2.3-11.2,9.6-20.9,19.8-26.1c2.1,3.3,4.2,6.7,6.3,10.3l0.4,0.7  c0.9,1.6,1.9,3.4,3.2,5.8l0.6,1.2C44.4,36.6,45.4,38.6,46.4,40.6z M24.4,51.1c2.2,0.1,4.2,0,6.2-0.1l0.7,0c0.4,0,0.9,0,1.3,0  c2.8-0.2,5.5-0.5,8.5-1c0.5-0.1,1-0.2,1.6-0.3l0.5-0.1c2.2-0.4,4.2-0.9,6.1-1.4c0.1,0,0.3-0.1,0.4-0.1l0.5,1.1  c1.2,2.8,2.3,5.5,3.3,8.1c0,0,0,0,0,0c-0.2,0.1-0.5,0.2-0.7,0.2c-2,0.6-4,1.4-5.9,2.2c-0.6,0.3-1.3,0.5-1.9,0.8  c-1.4,0.6-2.7,1.3-4.1,2.1l-0.3,0.2c-0.2,0.1-0.5,0.2-0.6,0.4c-1.5,0.9-3.1,1.9-4.7,3c-0.2,0.1-0.4,0.3-0.6,0.4  c-0.2,0.1-0.4,0.3-0.6,0.5c-1,0.7-2,1.5-3,2.3c-0.4,0.3-0.7,0.6-1.1,0.9l-0.3,0.3c-0.7,0.6-1.5,1.3-2.2,1.9l-0.2,0.2  c-0.4,0.4-0.7,0.7-1.1,1.1l-0.2,0.2c-0.6,0.6-1.3,1.3-2,2l-0.4,0.4c-0.2,0.2-0.4,0.4-0.5,0.6C16.1,69.9,12,60.2,12,50.3  c0,0,0.1,0,0.1,0c0.4,0,0.7,0,1.1,0.1c3.5,0.4,6.9,0.6,10.3,0.7C23.8,51,24.1,51.1,24.4,51.1z M29.5,81.9c0.2-0.2,0.3-0.4,0.5-0.5  c1-1.1,2-2.1,3-3c1.9-1.8,3.8-3.3,5.7-4.8c0.2-0.1,0.4-0.3,0.6-0.4c0.2-0.2,0.5-0.4,0.8-0.6c1.1-0.8,2.2-1.5,3.4-2.2  c0.1-0.1,0.2-0.1,0.3-0.2c0.1-0.1,0.2-0.1,0.3-0.2c1.4-0.8,2.9-1.6,4.5-2.3c0.3-0.1,0.6-0.2,0.8-0.4l0.6-0.3  c1.1-0.5,2.2-0.9,3.5-1.4c0.5-0.2,1.1-0.4,1.7-0.6l0.2-0.1c0.4-0.1,0.7-0.2,1.1-0.3c0,0,0,0,0,0c1.1,3.2,2.3,6.4,3.3,9.8l0.1,0.4  c1.1,3.6,2,7.3,2.9,10.8C51.7,89.8,39.3,88.3,29.5,81.9C29.4,81.9,29.4,81.9,29.5,81.9z M65.6,62.9c0.7-0.1,1.3-0.2,2-0.2  c2-0.2,4-0.2,5.9-0.2c0.2,0,0.4,0,0.6,0l0.2,0c2.2,0.1,4.6,0.3,6.9,0.6c0.4,0.1,0.9,0.1,1.3,0.2l0.6,0.1c0.7,0.1,1.5,0.3,2.2,0.4  c-3,7.6-8.3,14-15.2,18.3c-0.8-3.1-1.7-6.2-2.6-9.2l-0.1-0.4c-0.9-3-1.9-6.1-3.1-9.5C64.8,63.1,65.2,63,65.6,62.9z M81.6,55.2  C80,55,78.4,54.9,77,54.8l-0.9-0.1c-0.9-0.1-1.9-0.1-2.8-0.2c-0.2,0-0.3,0-0.5,0c-0.2,0-0.4,0-0.6,0c-2,0-3.9,0.1-5.9,0.3  c-0.2,0-0.3,0-0.5,0.1c-0.1,0-0.2,0-0.3,0c-1.3,0.1-2.6,0.3-3.9,0.5c-0.1-0.1-0.1-0.3-0.2-0.4c-0.1-0.2-0.2-0.5-0.3-0.7  c-1.1-2.9-2.3-5.7-3.2-7.8l-0.3-0.6c-0.1-0.1-0.1-0.3-0.2-0.4c0,0,0,0,0.1,0c0.2-0.1,0.5-0.2,0.7-0.3c0.6-0.2,1.2-0.5,1.8-0.8  c1.2-0.5,2.4-1.2,3.6-1.8c0.1-0.1,0.3-0.2,0.5-0.2c0.2-0.1,0.5-0.2,0.7-0.4c1.5-0.9,2.9-1.8,4.2-2.7l0.3-0.2  c0.2-0.1,0.4-0.3,0.6-0.4c0.9-0.6,1.9-1.4,2.8-2.2c1.5-1.2,2.9-2.5,4.3-4c0.8-0.8,1.5-1.6,2.2-2.4l0.4-0.5c0.5-0.5,0.9-1.1,1.3-1.6  C85.5,34.3,88,42.1,88,50c0,2-0.2,4.1-0.5,6.1c-0.3,0-0.6-0.1-0.8-0.1c-0.4-0.1-0.7-0.1-1.1-0.2l-1.1-0.2  C83.5,55.5,82.5,55.3,81.6,55.2z"
        ></path>
      </svg>
    </a>
    <div class="tooltip">Website</div>
  </li>
  <li class="icon-content">
    <a
      data-social="telegram"
      aria-label="Telegram"
      href="https://t.me/suppport2gc_bot"
      target="_blank"
    >
      <div class="filled"></div>
      <svg viewBox="0 0 100 100" version="1.1">
        <path
          fill="currentColor"
          d="M95,9.9c-1.3-1.1-3.4-1.2-7-0.1c0,0,0,0,0,0c-2.5,0.8-24.7,9.2-44.3,17.3c-17.6,7.3-31.9,13.7-33.6,14.5  c-1.9,0.6-6,2.4-6.2,5.2c-0.1,1.8,1.4,3.4,4.3,4.7c3.1,1.6,16.8,6.2,19.7,7.1c1,3.4,6.9,23.3,7.2,24.5c0.4,1.8,1.6,2.8,2.2,3.2  c0.1,0.1,0.3,0.3,0.5,0.4c0.3,0.2,0.7,0.3,1.2,0.3c0.7,0,1.5-0.3,2.2-0.8c3.7-3,10.1-9.7,11.9-11.6c7.9,6.2,16.5,13.1,17.3,13.9  c0,0,0.1,0.1,0.1,0.1c1.9,1.6,3.9,2.5,5.7,2.5c0.6,0,1.2-0.1,1.8-0.3c2.1-0.7,3.6-2.7,4.1-5.4c0-0.1,0.1-0.5,0.3-1.2  c3.4-14.8,6.1-27.8,8.3-38.7c2.1-10.7,3.8-21.2,4.8-26.8c0.2-1.4,0.4-2.5,0.5-3.2C96.3,13.5,96.5,11.2,95,9.9z M30,58.3l47.7-31.6  c0.1-0.1,0.3-0.2,0.4-0.3c0,0,0,0,0,0c0.1,0,0.1-0.1,0.2-0.1c0.1,0,0.1,0,0.2-0.1c-0.1,0.1-0.2,0.4-0.4,0.6L66,38.1  c-8.4,7.7-19.4,17.8-26.7,24.4c0,0,0,0,0,0.1c0,0-0.1,0.1-0.1,0.1c0,0,0,0.1-0.1,0.1c0,0.1,0,0.1-0.1,0.2c0,0,0,0.1,0,0.1  c0,0,0,0,0,0.1c-0.5,5.6-1.4,15.2-1.8,19.5c0,0,0,0,0-0.1C36.8,81.4,31.2,62.3,30,58.3z"
        ></path>
      </svg>
    </a>
    <div class="tooltip">Support</div>
  </li>
</ul>
        </form>
      {/if}
    {/if}
  </div>
  
</section>




<style>
  ul {
  list-style: none;
}

.example-2 {
  display: flex;
  justify-content: center;
  align-items: center;
}
.example-2 .icon-content {
  margin: 0 10px;
  position: relative;
}
.example-2 .icon-content .tooltip {
  position: absolute;
  top: -30px;
  left: 50%;
  transform: translateX(-50%);
  color: #fff;
  padding: 6px 10px;
  border-radius: 5px;
  opacity: 0;
  visibility: hidden;
  font-size: 14px;
  transition: all 0.3s ease;
}
.example-2 .icon-content:hover .tooltip {
  opacity: 1;
  visibility: visible;
  top: -50px;
}
.example-2 .icon-content a {
  position: relative;
  overflow: hidden;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 50px;
  height: 50px;
  border-radius: 50%;
  color: #4d4d4d;
  transition: all 0.3s ease-in-out;
}
.example-2 .icon-content a:hover {
  box-shadow: 3px 2px 45px 0px rgb(0 0 0 / 12%);
}
.example-2 .icon-content a svg {
  position: relative;
  z-index: 1;
  width: 30px;
  height: 30px;
}
.example-2 .icon-content a:hover {
  color: white;
}
.example-2 .icon-content a .filled {
  position: absolute;
  top: auto;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 0;
  background-color: #000;
  transition: all 0.3s ease-in-out;
}
.example-2 .icon-content a:hover .filled {
  height: 100%;
}
.example-2 .icon-content a[data-social="spotify"] .filled,
.example-2 .icon-content a[data-social="spotify"] ~ .tooltip {
  background-color: #1db954;
}
.example-2 .icon-content a[data-social="pinterest"] .filled,
.example-2 .icon-content a[data-social="pinterest"] ~ .tooltip {
  background-color: #bd081c;
}
.example-2 .icon-content a[data-social="dribbble"] .filled,
.example-2 .icon-content a[data-social="dribbble"] ~ .tooltip {
  background-color: #ea4c89;
}
.example-2 .icon-content a[data-social="telegram"] .filled,
.example-2 .icon-content a[data-social="telegram"] ~ .tooltip {
  background-color: #0088cc;
}

  .card {
  /* color used to softly clip top and bottom of the .words container */
  --bg-color: #212121;
  background-color: var(--bg-color);
  padding: 1rem 2rem;
  border-radius: 1.25rem;
}
.loader {
  color: rgb(124, 124, 124);
  font-family: "Poppins", sans-serif;
  font-weight: 500;
  font-size: 25px;
  -webkit-box-sizing: content-box;
  box-sizing: content-box;
  height: 40px;
  padding: 10px 10px;
  display: -webkit-box;
  display: -ms-flexbox;
  display: flex;
  border-radius: 8px;
}

.words {
  overflow: hidden;
  position: relative;
}
.words::after {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(
    var(--bg-color) 10%,
    transparent 30%,
    transparent 70%,
    var(--bg-color) 90%
  );
  z-index: 20;
}

.word {
  display: block;
  height: 100%;
  padding-left: 6px;
  color: #956afa;
  animation: spin_4991 4s infinite;
}

@keyframes spin_4991 {
  10% {
    -webkit-transform: translateY(-102%);
    transform: translateY(-102%);
  }

  25% {
    -webkit-transform: translateY(-100%);
    transform: translateY(-100%);
  }

  35% {
    -webkit-transform: translateY(-202%);
    transform: translateY(-202%);
  }

  50% {
    -webkit-transform: translateY(-200%);
    transform: translateY(-200%);
  }

  60% {
    -webkit-transform: translateY(-302%);
    transform: translateY(-302%);
  }

  75% {
    -webkit-transform: translateY(-300%);
    transform: translateY(-300%);
  }

  85% {
    -webkit-transform: translateY(-402%);
    transform: translateY(-402%);
  }

  100% {
    -webkit-transform: translateY(-400%);
    transform: translateY(-400%);
  }
}

  .notification {
    background: rgba(0, 128, 0, 0.1);
    color: #4caf50;
    border: 1px solid #4caf50;
    padding: 10px 15px;
    border-radius: 8px;
    margin-bottom: 1rem;
    text-align: center;
    font-size: 0.9rem;
  }

  .error-message {
    background: rgba(255, 0, 0, 0.1);
    color: #ff4c4c;
    border: 1px solid #ff4c4c;
    padding: 10px 15px;
    border-radius: 8px;
    margin-top: 0.5rem;
    text-align: center;
    font-size: 0.85rem;
  }

  .auth-links {
  display: flex;
  justify-content: space-between;
  margin-top: 1rem;
}

.auth-links a {
  color: #d3d3d3;
  font-size: 0.85rem;
  text-decoration: underline;
  transition: color 0.3s;
}

.auth-links a:hover {
  color: white;
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

.auth-form {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #121212;
  padding: 20px;
}

.auth-card {
  background: #171717;
  padding: 40px 30px;
  border-radius: 20px;
  width: 100%;
  max-width: 400px;
  /* Добавлен бордер */
  border: 2px solid #797979; /* Или любой градиент через background-image */
  box-shadow: inset 0 0 10px #0a0a0a, 0 0 20px rgba(0, 0, 0, 0.6);
  transition: transform 0.3s;
}


.auth-card:hover {
  transform: scale(1.02);
}

.auth-title {
  font-size: 1.8rem;
  font-weight: bold;
  text-align: center;
  color: rgb(197, 197, 197);
}

.auth-input {
  width: 100%;
  background: #2e2e2e;
  border: none;
  border-radius: 12px;
  padding: 12px 16px;
  color: #d3d3d3;
  outline: none;
  transition: background 0.3s;
}

.auth-input::placeholder {
  color: #777;
}

.auth-input:hover {
  background: #3a3a3a;
}

.auth-checkbox {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
  color: #d3d3d3;
}

.auth-button {
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

.auth-button:hover {
  background-color: black;
}

.auth-secondary-button {
  background-color: #2e2e2e;
  color: #d3d3d3;
  padding: 10px 20px;
  border-radius: 10px;
  transition: background 0.3s;
}

.auth-secondary-button:hover {
  background-color: #3a3a3a;
  color: white;
}

.auth-alert {
  background: rgba(255, 0, 0, 0.1);
  color: #ff4c4c;
  text-align: center;
  border-radius: 10px;
  padding: 10px;
  font-size: 0.8rem;
}

</style>