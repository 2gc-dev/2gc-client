<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { locale } from 'svelte-i18n';
  import { get } from 'svelte/store';

  export let autoUpdate: boolean = true;
  export let updateInterval: number = 30000; // 30 секунд
  export let speed: number = 14; // медленнее
  export let height: string = '45px';
  export let backgroundColor: string = '';
  export let textColor: string = '#e0e0e0';
  export let fontSize: string = '13px';
  export let fontWeight: string = '500';
  export let vertical: boolean = false;
  // export let lang: string = 'ru'; // теперь определяется автоматически

  const fallbackImg = 'https://2gc.ru/favicon.png';

  type Article = {
    title: string;
    url: string;
    date: string;
    summary: string;
    author: string;
    featured_image: string;
  };

  let news: Article[] = [];
  let tickerContainer: HTMLElement;
  let tickerContent: HTMLElement;
  let animationId: number;
  let isPaused: boolean = false;
  let position: number = 0;
  let containerSize: number = 0;
  let contentSize: number = 0;
  let updateTimer: number;
  let isLoading: boolean = false;
  let lang: string = 'ru';

  function getFullUrl(url: string) {
    if (url.startsWith('http')) return url;
    if (lang === 'en') return `https://2gc.io${url}`;
    return `https://2gc.ru${url}`;
  }

  function getImageUrl(img: string) {
    if (!img) return fallbackImg;
    if (img.startsWith('http')) return img;
    if (lang === 'en') return `https://2gc.io${img}`;
    return `https://2gc.ru${img}`;
  }

  function handleImgError(e: Event) {
    (e.target as HTMLImageElement).src = fallbackImg;
  }

  async function fetchBlogArticles(lang = 'ru') {
    try {
      const response = await fetch(`https://2gc.ru/api/blog-preview-${lang}.json`);
      const data = await response.json();
      if (data.status === 'success') {
        return data.data.articles;
      } else {
        throw new Error('API error');
      }
    } catch (error) {
      console.error('Ошибка получения данных:', error);
      return [];
    }
  }

  async function fetchSecurityNews() {
    isLoading = true;
    try {
      const articles = await fetchBlogArticles(lang);
      news = articles.slice(0, 8).map(a => ({
        title: a.title,
        url: a.url,
        date: a.date,
        summary: a.summary,
        author: a.author,
        featured_image: a.featured_image
      }));
      console.log('Новости блога:', news);
      updateDimensions();
      restartAnimation();
    } catch (e) {
      // ...
    } finally {
      isLoading = false;
    }
  }

  function startAnimation() {
    if (isPaused) return;
    const animate = () => {
      if (vertical) {
        position += speed / 60; // сверху вниз
        if (position >= containerSize) {
          position = -contentSize;
        }
        if (tickerContent) {
          tickerContent.style.transform = `translateY(${position}px)`;
        }
      } else {
        position -= speed / 60; // слева направо
        if (position <= -contentSize) {
          position = containerSize;
        }
        if (tickerContent) {
          tickerContent.style.transform = `translateX(${position}px)`;
        }
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

  function restartAnimation() {
    stopAnimation();
    startAnimation();
  }

  function handleMouseEnter() {
    isPaused = true;
    stopAnimation();
  }

  function handleMouseLeave() {
    isPaused = false;
    startAnimation();
  }

  function updateDimensions() {
    if (tickerContainer && tickerContent) {
      if (vertical) {
        containerSize = tickerContainer.offsetHeight;
        contentSize = tickerContent.offsetHeight;
        position = -contentSize;
      } else {
        containerSize = tickerContainer.offsetWidth;
        contentSize = tickerContent.offsetWidth;
        position = containerSize;
      }
    }
  }

  function handleRefresh() {
    fetchSecurityNews();
  }

  // Следим за языком приложения
  $: {
    const $locale = get(locale);
    if ($locale === 'en') lang = 'en';
    else lang = 'ru';
  }
  // При смене lang — подгружаем новые статьи
  $: if (lang) {
    fetchSecurityNews();
  }

  onMount(() => {
    // lang определяется выше
    fetchSecurityNews();
    updateDimensions();
    startAnimation();
    if (autoUpdate) {
      updateTimer = setInterval(fetchSecurityNews, updateInterval);
    }
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
    if (updateTimer) {
      clearInterval(updateTimer);
    }
  });

  $: if (speed !== undefined) {
    stopAnimation();
    startAnimation();
  }
</script>

<div class="advanced-news-ticker {vertical ? 'vertical-ticker-float' : ''}" style="{vertical ? 'width: 360px;' : ''}">
  <!-- <div class="ticker-header">
    <span class="ticker-label">📰 Новости блога 2GC</span>
    <button 
      class="refresh-button" 
      on:click={handleRefresh}
      disabled={isLoading}
      title="Обновить новости"
    >
      {#if isLoading}
        <span class="loading-spinner">⟳</span>
      {:else}
        <span>⟳</span>
      {/if}
    </button>
  </div> -->
  <div 
    class="ticker-container {vertical ? 'vertical' : ''}"
    bind:this={tickerContainer}
    on:mouseenter={handleMouseEnter}
    on:mouseleave={handleMouseLeave}
    style="
      background-color: {backgroundColor};
      color: {textColor};
      font-size: {fontSize};
      font-weight: {fontWeight};
      {vertical ? 'height: 340px; min-height: 220px; max-height: 550px;' : ''}
    "
  >
    <div 
      class="ticker-content"
      bind:this={tickerContent}
      style="{vertical ? 'display: flex; flex-direction: column;' : ''}"
    >
      {#each news as item}
        <a class="ticker-item blog-preview" href={getFullUrl(item.url)} target="_blank" rel="noopener">
          <div class="blog-img-wrap">
            <img class="blog-img" src={getImageUrl(item.featured_image)} alt={item.title} loading="lazy" on:error={handleImgError} />
          </div>
          <div class="blog-info">
            <div class="blog-title">{item.title}</div>
            <div class="blog-meta">
              <span class="blog-author"><svg width="16" height="16" fill="none" viewBox="0 0 24 24"><circle cx="12" cy="8" r="4" fill="#7ecfff"/><path d="M4 20c0-2.21 3.58-4 8-4s8 1.79 8 4" stroke="#7ecfff" stroke-width="1.5" stroke-linecap="round"/></svg> {item.author}</span>
              <span class="blog-date"><svg width="16" height="16" fill="none" viewBox="0 0 24 24"><rect x="3" y="5" width="18" height="16" rx="2" fill="#aaa"/><path d="M16 3v4M8 3v4M3 9h18" stroke="#fff" stroke-width="1.5" stroke-linecap="round"/></svg> {item.date}</span>
            </div>
          </div>
        </a>
      {/each}
      {#if vertical}
        {#each news as item}
          <a class="ticker-item blog-preview" href={getFullUrl(item.url)} target="_blank" rel="noopener">
            <div class="blog-img-wrap">
              <img class="blog-img" src={getImageUrl(item.featured_image)} alt={item.title} loading="lazy" on:error={handleImgError} />
            </div>
            <div class="blog-info">
              <div class="blog-title">{item.title}</div>
              <div class="blog-meta">
                <span class="blog-author"><svg width="16" height="16" fill="none" viewBox="0 0 24 24"><circle cx="12" cy="8" r="4" fill="#7ecfff"/><path d="M4 20c0-2.21 3.58-4 8-4s8 1.79 8 4" stroke="#7ecfff" stroke-width="1.5" stroke-linecap="round"/></svg> {item.author}</span>
                <span class="blog-date"><svg width="16" height="16" fill="none" viewBox="0 0 24 24"><rect x="3" y="5" width="18" height="16" rx="2" fill="#aaa"/><path d="M16 3v4M8 3v4M3 9h18" stroke="#fff" stroke-width="1.5" stroke-linecap="round"/></svg> {item.date}</span>
              </div>
            </div>
          </a>
        {/each}
      {/if}
    </div>
  </div>
</div>

<style>
  .advanced-news-ticker {
    display: flex;
    flex-direction: column;
    gap: 2px;
    z-index: 1;
    width: 100%;
    background: none;
    box-shadow: none;
    border: none;
    padding: 0;
  }
  .ticker-container {
    position: relative;
    overflow: visible;
    display: flex;
    align-items: center;
    border-radius: 0;
    padding: 0;
    box-shadow: none;
    border: none;
    min-width: 0;
    background: none;
  }
  .ticker-container.vertical {
    flex-direction: column;
    align-items: flex-end;
    height: 220px;
    min-height: 60px;
    max-height: 220px;
    width: 100%;
    padding: 8px 0;
    overflow: visible;
  }
  .ticker-content {
    display: flex;
    align-items: center;
    white-space: nowrap;
    will-change: transform;
    width: 100%;
  }
  .ticker-container.vertical .ticker-content {
    flex-direction: column;
    align-items: flex-start;
    white-space: normal;
    width: 70%;
    overflow: visible;
  }
  .ticker-item.blog-preview {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    gap: 7px;
    background: rgba(30,30,40,0.65);
    color: #e6e6e6;
    font-size: 0.78rem;
    font-weight: 400;
    border-radius: 8px;
    padding: 8px 8px 8px 0;
    width: 100%;
    min-height: 22px;
    text-decoration: none;
    transition: all 0.2s ease;
    margin-bottom: 8px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    opacity: 0.97;
    will-change: background, transform;
    border: 1px solid rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    overflow: visible;
  }
  .ticker-item.blog-preview:hover {
    background: rgba(80, 160, 255, 0.15);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15), 0 0 0 1px rgba(255, 255, 255, 0.2);
    transform: translateY(-1px);
    opacity: 1;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }
  .blog-img-wrap {
    width: 38px;
    height: 22px;
    border-radius: 6px;
    overflow: hidden;
    background: rgba(35, 35, 35, 0.8);
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  .blog-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    border-radius: 6px;
    display: block;
    background: rgba(34, 34, 34, 0.8);
  }
  .blog-info {
    overflow: visible;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 3px;
    flex: 1;
    padding-right: 4px;
  }
  .blog-title {
    font-size: 0.82rem;
    font-weight: 300;
    color: #adadad;
    margin-bottom: 0;
    line-height: 1.2;
    white-space: normal;
    word-break: break-word;
    overflow: visible;
    text-overflow: unset;
    max-width: 100%;
    hyphens: auto;
  }
  .ticker-item.blog-preview:hover .blog-title {
    text-decoration: underline dotted #4b9eff;
    color: #4b9eff;
  }
  .blog-summary {
    font-size: 0.74rem;
    color: #b0b0b0;
    line-height: 1.12;
    max-height: none;
    overflow: visible;
    text-overflow: unset;
    white-space: normal;
    margin-bottom: 0;
  }
  .blog-meta {
    display: flex;
    flex-direction: row;
    gap: 6px;
    font-size: 0.68rem;
    color: #8bb;
    margin-top: 2px;
    align-items: center;
    opacity: 0.85;
    flex-wrap: wrap;
  }
  .blog-author, .blog-date {
    display: flex;
    align-items: center;
    gap: 3px;
    color: #7ecfff;
    font-weight: 500;
    font-size: 0.74em;
    white-space: nowrap;
  }
  .blog-date {
    color: #aaa;
    font-weight: 400;
  }
  .blog-author svg, .blog-date svg {
    margin-right: 0;
    margin-bottom: 0;
    vertical-align: middle;
    flex-shrink: 0;
  }
  /* Fade-градиенты сверху и снизу */
  .blog-ticker-gradient-top,
  .blog-ticker-gradient-bottom {
    content: '';
    position: absolute;
    left: 0; right: 0;
    height: 28px;
    z-index: 2;
    pointer-events: none;
  }
  .blog-ticker-gradient-top {
    top: 0;
    background: linear-gradient(to bottom, rgba(30,30,40,0.82) 80%, transparent 100%);
  }
  .blog-ticker-gradient-bottom {
    bottom: 0;
    background: linear-gradient(to top, rgba(30,30,40,0.82) 80%, transparent 100%);
  }
</style> 