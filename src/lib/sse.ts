// src/lib/sse.ts
import { invoke } from '@tauri-apps/api/core';
import { USER_AGENT } from './constants';

export function subscribeToSSE(conversation_id: string) {
  const url = `http://localhost:4000/events?conversation_id=${conversation_id}&user_agent=${encodeURIComponent(USER_AGENT)}`;

  const source = new EventSource(url);

  source.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      console.log('📩 Получено событие SSE:', data);

      const title = `🔔 Новое сообщение от ${data.sender}`;
      const body = data.message;

      invoke('show_notification', { title, body });
    } catch (e) {
      console.error('Ошибка обработки SSE события:', e);
    }
  };

  source.onerror = (err) => {
    console.error('SSE ошибка:', err);
  };

  return source;
}
