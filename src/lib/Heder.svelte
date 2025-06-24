<script>
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { invoke } from '@tauri-apps/api/core';
  import { locale } from 'svelte-i18n';
  import { platform } from '@tauri-apps/plugin-os';

  const appWindow = getCurrentWebviewWindow();

let isWindows = false;
let isMacOS = false;

onMount(async () => {
  const p = await platform();
  isWindows = p === 'windows';
  isMacOS = p === 'macos';
  // можно console.log(p, isMacOS, isWindows);
});



  async function stopCloudflaredProcesses() {
    try {
      await invoke('stop_all_cloudflared_processes_frontend');
      console.log("✅ Запрос на остановку cloudflared процессов отправлен (при закрытии окна).");
    } catch (error) {
      console.error("❌ Ошибка при отправке запроса на остановку cloudflared (при закрытии окна):", error);
    }
  }

  onDestroy(() => {
    stopCloudflaredProcesses();
  });
</script>
<div class="base-head">
  <div class="head" data-tauri-drag-region>
    <div class="left-controls">
      {#if isMacOS}
        <div class="window-controls">
          <div class="mac-dot red" on:click={() => appWindow.close()}></div>
          <div class="mac-dot yellow" on:click={() => appWindow.minimize()}></div>
        </div>
      {/if}
    </div>
    <div class="text-logo-wrapper">
      <div class="text-logo">
        <span class="shine">2GC CLOUDBRIDGE</span>
      </div>
    </div>
    <div class="right-controls flex items-center gap-4 text-gray-400 language-switcher-wrapper">
      <div class="language-switcher">
        {#if $locale === 'ru'}
          <button class="flag-btn" on:click={() => locale.set('en')} title="Switch to English">RU</button>
        {:else}
          <button class="flag-btn" on:click={() => locale.set('ru')} title="Переключить на русский">EN</button>
        {/if}
      </div>
      {#if isWindows}
        <div class="hover:text-gray-200 cursor-pointer" on:click={() => appWindow.minimize()}>
          <i class="fas fa-minus"></i>
        </div>
        <div class="hover:text-gray-200 cursor-pointer" on:click={() => appWindow.close()}>
          <i class="fas fa-times"></i>
        </div>
      {/if}
    </div>
  </div>
</div>

   <style>
.window-controls {
  display: flex;
  gap: 8px;
  padding-left: 12px;
  align-items: center;
  height: 100%;
}

.mac-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  cursor: pointer;
  box-shadow: 0 0 0 1px rgba(0,0,0,0.1);
}

.mac-dot.red {
  background-color: #ff5f57;
}

.mac-dot.yellow {
  background-color: #ffbd2e;
}

.mac-dot:hover {
  opacity: 0.8;
}

.shine {
  font-size: 26px;
  font-weight: 900;
  color: rgba(255, 255, 255, 0.3);
  background: #222 -webkit-gradient(
      linear,
      left top,
      right top,
      from(#222),
      to(#222),
      color-stop(0.5, #fff)
    ) 0 0 no-repeat;
  background-image: -webkit-linear-gradient(
    -40deg,
    transparent 0%,
    transparent 40%,
    #fff 50%,
    transparent 60%,
    transparent 100%
  );
  -webkit-background-clip: text;
  -webkit-background-size: 50px;
  -webkit-animation: zezzz;
  -webkit-animation-duration: 5s;
  -webkit-animation-iteration-count: infinite;
}
@-webkit-keyframes zezzz {
  0%,
  10% {
    background-position: -200px;
  }
  20% {
    background-position: top left;
  }
  100% {
    background-position: 200px;
  }
}

    
.head {
  display: flex;
  justify-content: space-between; /* Края! */
  align-items: center; 
  height: 50px;
  position: relative;
  padding: 0 10px;
  box-sizing: border-box; 
}

.left-controls, .right-controls {
  min-width: 70px; /* чтобы не прыгало */
  display: flex;
  align-items: center;
  height: 100%;
}

.text-logo-wrapper {
  position: absolute;
  left: 50%;
  top: 0;
  transform: translateX(-50%);
  height: 100%;
  display: flex;
  align-items: center;
  z-index: 1;
}


.text-logo {
  font-family: 'Inter', sans-serif;
  font-size: 15px;
  font-weight: 600;
  color: #d0d1f8;

  padding: 4px 12px;
  border-radius: 12px;
  display: inline-block;
  user-select: none;
}

.logo-light {
  opacity: 0.8;
}

.logo-bold {
  font-weight: 700;
}


	/* Rectangle 9 */
.head {
    position: absolute;
    width: calc(100% - 10px); /* Уменьшаем ширину на ext-logo-wrapp пикселей (5px с каждой стороны) */
    display: flex;
    border-radius: 0px 0px 2px 2px;
    justify-content: space-between;
    align-items: center;
    padding: 0 5px; /* Добавляем отступы по 5 пикселей с каждой стороны */
    box-sizing: border-box; /* Учитываем отступы в ширине контейнера */
}



.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}

.logo-1 {
    margin-left: 50px;
}
.logo-2 {
    display: flex;
    align-items: flex-start; /* Выровнять по верхнему краю контейнера */
    height: 100%; /* Занимает всю высоту контейнера */
    cursor: pointer; /* Изменяет курсор при наведении */
    transition: background-color 0.3s, transform 0.3s; /* Плавный переход фона и эффекта увеличения */
    margin-top: 30px; /* Смещает иконку вниз на 15px */
}
/* Эффект при наведении */
.logo-2:hover {
    transform: scale(1.25); /* Легкое увеличение при наведении */
}

/* Чтобы исправить выравнивание иконки */
.logo-2 svg {
    vertical-align: middle; /* Выравнивание по середине по вертикали */
}

  </style>