# 2GC Cloudflare Worker
<div id="header" align="center">
    <img src="https://pub-a89b5697d4074daeb851dc6c011ed225.r2.dev/2gc_logo.ico" alt="2GC Logo">
</div>
<div id="badges" align="center">
    [![License](https://img.shields.io/github/license/mlanies/2GC?style=for-the-badge)](https://github.com/mlanies/2GC/blob/main/LICENSE)
    [![Website](https://img.shields.io/badge/Website-000000?style=for-the-badge&logoColor=white)](https://2gc.ru)
    [![Telegram Support](https://img.shields.io/badge/Telegram%20Support-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)](https://t.me/suppport2gc_bot)
    [![Stars](https://img.shields.io/github/stars/mlanies/2gc-cloudflare-worker?style=for-the-badge)](https://github.com/mlanies/2gc-cloudflare-worker/stargazers)
    [![Forks](https://img.shields.io/github/forks/mlanies/2gc-cloudflare-worker?style=for-the-badge)](https://github.com/mlanies/2gc-cloudflare-worker/network)
    [![Issues](https://img.shields.io/github/issues/mlanies/2gc-cloudflare-worker?style=for-the-badge)](https://github.com/mlanies/2gc-cloudflare-worker/issues)
</div>

## О проекте

2GC Cloudflare Worker - это приложение, которое позволяет легко настроить и управлять Cloudflare Workers, создавая безопасное и производительное соединение между вашими сервисами и Cloudflare.

### Преимущества

* **Безопасность**: Интеграция с Aikido для мониторинга и защиты от угроз
* **Производительность**: Оптимизированная работа с Cloudflare Workers
* **Удобство**: Современный интерфейс без сложных команд и настроек
* **Надежность**: Нативный бэкенд на Rust для максимальной стабильности

## 🛡️ Варианты подключения

2GC поддерживает два способа туннелирования:

- **Cloudflare Tunnel** — стандартный облачный способ.
- **2GC Relay** — собственный сервер туннелирования, реализующий защищённый протокол обмена.

### Протокол 2GC Relay

- Взаимодействие по TCP с обязательным TLS 1.3.
- Аутентификация через JWT.
- Регистрация туннеля с выдачей уникального tunnel_id и конфигурации.
- Поддержка heartbeat, мониторинга, автоматического восстановления соединения.

Подробнее: [docs/RELAY_PROTOCOL.md](docs/RELAY_PROTOCOL.md)

## 🚀 Быстрый старт

### Системные требования
- Node.js >= 18
- pnpm (или npm/yarn)
- Rust (для desktop/Tauri)
- Windows 10/11 или macOS (WebView2 для Windows)

---

## 🔧 Запуск для разработчиков

### 1. Клонирование репозитория
```bash
git clone https://github.com/mlanies/2gc-cloudflare-worker.git
cd 2gc-cloudflare-worker
```

### 2. Установка зависимостей (Frontend)
```bash
pnpm install
# или npm install
```

### 3. Запуск веб-приложения (Svelte/Vite)
```bash
pnpm run dev
# или npm run dev
```

### 4. Запуск desktop-приложения (Tauri)
- Установите [Rust](https://www.rust-lang.org/tools/install) и [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/):
```bash
cargo install tauri-cli
```
- Запустите desktop-версию:
```bash
pnpm tauri dev
# или npm run tauri dev
```

---

## 🏗️ Сборка и деплой

### Сборка веб-приложения
```bash
pnpm run build
# или npm run build
```

### Сборка desktop-приложения (Tauri)
```bash
pnpm tauri build
# или npm run tauri build
```

- Бинарные файлы появятся в `src-tauri/target/release/` (или соответствующей папке).

---

## 2GC CloudBridge

[GitHub](https://github.com/twogc/2gc-business)

Расширенная версия с дополнительными возможностями для бизнеса.

[![2GC CloudBridge](https://pub-a89b5697d4074daeb851dc6c011ed225.r2.dev/2gc_app_list.svg)](https://2gc.ru/download)

**Функции 2GC CloudBridge**

* Управление Cloudflare Workers
* Мониторинг производительности
* Расширенная отладка
* Интеграция с системой безопасности Aikido
* Сбор и анализ клиентской информации
* Поддержка различных типов клиентов

## Личный кабинет 2GC CloudBridge

Управляйте доступом и серверами из централизованного интерфейса.

[![Личный кабинет 2GC CloudBridge](https://pub-a89b5697d4074daeb851dc6c011ed225.r2.dev/lk_2gc.png)](https://2gc.ru/download)

## 👥 Contributors

<a href="https://github.com/mlanies/2GC/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=mlanies/2GC" />
</a>

## 📜 License

2GC Cloudflare Worker распространяется под лицензией [MIT](LICENSE).

## 11. Техническая реализация

### 11.1. Архитектура приложения

#### 11.1.1. Frontend (Svelte)
- Современный пользовательский интерфейс на Svelte
- Адаптивный дизайн с поддержкой темной темы
- Многоязычный интерфейс (русский/английский)
- Интуитивная навигация и управление
- Система уведомлений и статусов

#### 11.1.2. Backend (Rust/Tauri)
- Нативный десктопный клиент на Rust
- Кроссплатформенная поддержка (Windows, macOS)
- Системный трей с быстрым доступом
- Управление процессами и туннелями
- Безопасное хранение учетных данных

#### 11.1.3. Туннелирование
- Поддержка различных типов туннелей:
  - Cloudflare Tunnel
  - Bore Tunnel
- Автоматическое управление соединениями
- Мониторинг состояния туннелей
- Автоматическое восстановление при разрывах

### 11.2. Компоненты системы

#### 11.2.1. Основные модули
- `main.rs` - точка входа приложения
- `process.rs` - управление процессами
- `rdp_settings.rs` - настройки RDP-подключений
- `ssh_settings.rs` - настройки SSH-подключений
- `storage.rs` - безопасное хранение данных
- `config.rs` - конфигурация приложения

#### 11.2.2. Безопасность
- Шифрование данных в хранилище
- Безопасное управление учетными данными
- Проверка подписи JWT-токенов
- Защита от несанкционированного доступа

#### 11.2.3. Интеграции
- Поддержка RDP-клиентов
- Интеграция с SSH-клиентами
- API для внешних систем
- Система обновлений

### 11.3. Технические особенности

#### 11.3.1. Управление процессами
- Автоматический запуск/остановка туннелей
- Мониторинг состояния процессов
- Обработка ошибок и восстановление
- Логирование событий

#### 11.3.2. Хранение данных
- Безопасное хранение учетных данных
- Кэширование настроек
- Управление конфигурацией
- Сохранение состояния приложения

#### 11.3.3. Пользовательский интерфейс
- Современный дизайн
- Адаптивная верстка
- Система уведомлений
- Управление состоянием

### 11.4. Развертывание

#### 11.4.1. Требования
- Windows 10/11 или macOS
- Поддержка WebView2 (Windows)
- Доступ к интернету
- Права администратора (для установки)

#### 11.4.2. Установка
- Автоматическая установка зависимостей
- Настройка системных компонентов
- Создание ярлыков и интеграция
- Первоначальная конфигурация

#### 11.4.3. Обновления
- Автоматическая проверка обновлений
- Безопасное обновление компонентов
- Сохранение пользовательских настроек
- Откат при ошибках

### 11.5. Мониторинг и поддержка

#### 11.5.1. Логирование
- Системные события
- Ошибки и предупреждения
- Действия пользователя
- Состояние соединений

#### 11.5.2. Диагностика
- Проверка состояния туннелей
- Тестирование соединений
- Анализ производительности
- Отчеты о проблемах

#### 11.5.3. Поддержка
- Автоматическая диагностика
- Система уведомлений
- Инструкции по устранению проблем
- Обратная связь
