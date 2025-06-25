<script lang="ts">
  import NewsTicker from './NewsTicker.svelte';
  import AdvancedNewsTicker from './AdvancedNewsTicker.svelte';

  // Примеры новостей для демонстрации
  const securityNews = [
    "🔒 Ваше соединение защищено шифрованием AES-256",
    "🛡️ Обнаружена попытка несанкционированного доступа - заблокировано",
    "✅ Все серверы работают стабильно",
    "📊 Статистика безопасности: 99.9% uptime за последний месяц"
  ];

  const systemNews = [
    "⚡ Обновление системы безопасности завершено успешно",
    "🌐 Подключение к серверу через защищенный туннель",
    "📱 Двухфакторная аутентификация активна",
    "🚨 Новое обновление безопасности доступно"
  ];

  const generalNews = [
    "Добро пожаловать в 2GC CloudBridge",
    "Система мониторинга активна",
    "Все сервисы работают в штатном режиме",
    "Техническая поддержка доступна 24/7"
  ];
</script>

<div class="ticker-examples">
  <h2>Примеры бегущих строк</h2>
  
  <!-- Базовый тикер -->
  <div class="example-section">
    <h3>Базовый NewsTicker</h3>
    <NewsTicker 
      news={securityNews}
      speed={50}
      height="40px"
      backgroundColor="#1a1a1a"
      textColor="#ffffff"
      fontSize="14px"
      fontWeight="400"
    />
  </div>

  <!-- Продвинутый тикер -->
  <div class="example-section">
    <h3>Продвинутый AdvancedNewsTicker</h3>
    <AdvancedNewsTicker 
      speed={40}
      height="45px"
      textColor="#e0e0e0"
      fontSize="13px"
      fontWeight="500"
      autoUpdate={true}
      updateInterval={30000}
    />
  </div>

  <!-- Тонкий тикер для заголовка -->
  <div class="example-section">
    <h3>Тонкий тикер для заголовка</h3>
    <NewsTicker 
      news={generalNews}
      speed={30}
      height="35px"
      backgroundColor="transparent"
      textColor="#888888"
      fontSize="12px"
      fontWeight="300"
      pauseOnHover={false}
    />
  </div>

  <!-- Быстрый тикер справа налево -->
  <div class="example-section">
    <h3>Быстрый тикер справа налево</h3>
    <NewsTicker 
      news={systemNews}
      speed={80}
      height="40px"
      backgroundColor="#2a2a2a"
      textColor="#00ff88"
      fontSize="13px"
      fontWeight="500"
      direction="right"
    />
  </div>
</div>

<style>
  .ticker-examples {
    padding: 20px;
    background: #0f0f0f;
    border-radius: 12px;
    margin: 20px 0;
  }

  .ticker-examples h2 {
    color: #e0e0e0;
    font-size: 1.5rem;
    margin-bottom: 20px;
    text-align: center;
  }

  .example-section {
    margin-bottom: 30px;
  }

  .example-section h3 {
    color: #a0a0a0;
    font-size: 1rem;
    margin-bottom: 10px;
    font-weight: 500;
  }

  .example-section:last-child {
    margin-bottom: 0;
  }
</style> 