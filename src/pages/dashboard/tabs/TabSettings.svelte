<script lang="ts">
  import { settings, saveSettings, resetSettings } from '../../../store/settings';
  import { t } from 'svelte-i18n';
</script>

<div class="settings-container">
  <!-- Настройки интерфейса -->
  <div class="settings-section">
    <h3 class="settings-title">{$t('interface_settings')}</h3>
    <div class="settings-group">
      <label class="settings-label">
        <span>{$t('theme')}</span>
        <select class="settings-select" bind:value={$settings.theme}>
          <option value="dark">{$t('dark_theme')}</option>
          <option value="light">{$t('light_theme')}</option>
          <option value="system">{$t('system_theme')}</option>
        </select>
      </label>
      <label class="settings-label">
        <span>{$t('language')}</span>
        <select class="settings-select" bind:value={$settings.language}>
          <option value="ru">Русский</option>
          <option value="en">English</option>
        </select>
      </label>
    </div>
  </div>
  <!-- ...и так далее для остальных групп настроек... -->
  <div class="settings-actions">
    <button class="settings-button save" on:click={saveSettings}>{$t('save_settings')}</button>
    <button class="settings-button reset" on:click={resetSettings}>{$t('reset_settings')}</button>
  </div>
</div>
