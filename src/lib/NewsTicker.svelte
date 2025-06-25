<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  export let news: string[] = [
    "🔒 Ваше соединение защищено шифрованием AES-256",
    "🛡️ Обнаружена попытка несанкционированного доступа - заблокировано",
    "✅ Все серверы работают стабильно",
    "📊 Статистика безопасности: 99.9% uptime за последний месяц",
    "🔐 Рекомендуется сменить пароль каждые 90 дней",
    "⚡ Обновление системы безопасности завершено успешно",
    "🌐 Подключение к серверу через защищенный туннель",
    "📱 Двухфакторная аутентификация активна"
  ];
  
  export let speed: number = 50; // пикселей в секунду
  export let pauseOnHover: boolean = true;
  export let loop: boolean = true;
  export let direction: 'left' | 'right' = 'left';
  export let height: string = '40px';
  export let backgroundColor: string = '#1a1a1a';
  export let textColor: string = '#ffffff';
  export let fontSize: string = '14px';
  export let fontWeight: string = '400';

  let tickerContainer: HTMLElement;
  let tickerContent: HTMLElement;
  let animationId: number;
  let isPaused: boolean = false;
  let position: number = 0;
  let containerWidth: number = 0;
  let contentWidth: number = 0;

  function startAnimation() {
    if (isPaused) return;
    
    const animate = () => {
      if (direction === 'left') {
        position -= speed / 60; // 60 FPS
        if (position <= -contentWidth && loop) {
          position = containerWidth;
        } else if (position <= -contentWidth && !loop) {
          return;
        }
      } else {
        position += speed / 60;
        if (position >= containerWidth && loop) {
          position = -contentWidth;
        } else if (position >= containerWidth && !loop) {
          return;
        }
      }
      
      if (tickerContent) {
        tickerContent.style.transform = `translateX(${position}px)`;
      }
      
      animationId = requestAnimationFrame(animate);
    };
    
    animationId = requestAnimationFrame(animate);
  }

  function stopAnimation() {
    if (animationId) {
      cancelAnimationFrame(animationId);
    }
  }

  function handleMouseEnter() {
    if (pauseOnHover) {
      isPaused = true;
      stopAnimation();
    }
  }

  function handleMouseLeave() {
    if (pauseOnHover) {
      isPaused = false;
      startAnimation();
    }
  }

  function updateDimensions() {
    if (tickerContainer && tickerContent) {
      containerWidth = tickerContainer.offsetWidth;
      contentWidth = tickerContent.offsetWidth;
      
      if (direction === 'left') {
        position = containerWidth;
      } else {
        position = -contentWidth;
      }
    }
  }

  onMount(() => {
    updateDimensions();
    startAnimation();
    
    // Обновляем размеры при изменении размера окна
    const resizeObserver = new ResizeObserver(updateDimensions);
    if (tickerContainer) {
      resizeObserver.observe(tickerContainer);
    }
    
    return () => {
      resizeObserver.disconnect();
    };
  });

  onDestroy(() => {
    stopAnimation();
  });

  // Перезапускаем анимацию при изменении параметров
  $: if (speed !== undefined) {
    stopAnimation();
    startAnimation();
  }

  $: if (direction !== undefined) {
    updateDimensions();
    stopAnimation();
    startAnimation();
  }
</script>

<div 
  class="news-ticker"
  bind:this={tickerContainer}
  on:mouseenter={handleMouseEnter}
  on:mouseleave={handleMouseLeave}
  style="
    height: {height};
    background-color: {backgroundColor};
    color: {textColor};
    font-size: {fontSize};
    font-weight: {fontWeight};
  "
>
  <div 
    class="ticker-content"
    bind:this={tickerContent}
  >
    {#each news as item, index}
      <span class="ticker-item">
        {item}
        {#if index < news.length - 1}
          <span class="ticker-separator"> • </span>
        {/if}
      </span>
    {/each}
  </div>
</div>

<style>
  .news-ticker {
    position: relative;
    overflow: hidden;
    display: flex;
    align-items: center;
    border-radius: 8px;
    padding: 0 16px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    border: 1px solid #333;
  }

  .ticker-content {
    display: flex;
    align-items: center;
    white-space: nowrap;
    will-change: transform;
  }

  .ticker-item {
    display: inline-block;
    margin-right: 8px;
  }

  .ticker-separator {
    color: #666;
    margin: 0 8px;
  }

  /* Градиентные края для плавного появления/исчезновения */
  .news-ticker::before,
  .news-ticker::after {
    content: '';
    position: absolute;
    top: 0;
    bottom: 0;
    width: 40px;
    z-index: 1;
    pointer-events: none;
  }

  .news-ticker::before {
    left: 0;
    background: linear-gradient(to right, var(--bg-color, #1a1a1a), transparent);
  }

  .news-ticker::after {
    right: 0;
    background: linear-gradient(to left, var(--bg-color, #1a1a1a), transparent);
  }
</style> 