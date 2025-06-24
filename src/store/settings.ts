import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export const settings = writable({
  theme: 'system',
  language: 'ru',
  enableNotifications: true,
  connectionNotifications: true,
  autoUpdate: false,
  startWithSystem: false,
  saveCredentials: false,
  // ... другие дефолты!
});

// Загрузить настройки из Rust (асинхронно)
export async function loadSettings() {
  try {
    const data = await invoke('load_settings');
    if (data && typeof data === 'object') {
      settings.update(current => ({ ...current, ...data }));
    }
  } catch (e) {
    console.warn('Ошибка загрузки настроек:', e);
  }
}

// Сохранить настройки в Rust
export async function saveSettings() {
  let value;
  settings.subscribe(v => value = v)(); // Получить текущее значение store
  try {
    await invoke('save_settings', { settings: value });
  } catch (e) {
    console.warn('Ошибка сохранения настроек:', e);
  }
}
