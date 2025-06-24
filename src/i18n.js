import { register, init, locale as $locale } from 'svelte-i18n';

register('en', () => import('./locales/en.json'));
register('ru', () => import('./locales/ru.json'));

export async function setupI18n() {
  let systemLocale = 'en';
  try {
    // Проверяем Tauri-режим
    if (window.__TAURI__) {
      const { locale } = await import('@tauri-apps/plugin-os');
      const loc = await locale();
      if (loc && loc.startsWith('ru')) systemLocale = 'ru';
      if (loc && loc.startsWith('en')) systemLocale = 'en';
    } else {
      // Веб-браузер
      const { getLocaleFromNavigator } = await import('svelte-i18n');
      systemLocale = getLocaleFromNavigator() || 'en';
    }
  } catch {
    systemLocale = 'en';
  }
  init({
    fallbackLocale: 'en',
    initialLocale: systemLocale
  });
  $locale.set(systemLocale);
}
