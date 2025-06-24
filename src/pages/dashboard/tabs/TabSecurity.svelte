<script lang="ts">
  import { t } from 'svelte-i18n';
  // Импортируй, что нужно для секьюрити-отчёта/отображения
</script>

<div class="card">
  <img src="https://uiverse.io/astronaut.png" alt="" class="image" />
  <div class="text-white font-semibold">
    {$t('security_greeting')}, {$user?.name ?? 'Гость'}{$t('security_greeting_suffix')}
  </div>
  <!-- Любой твой контент -->
</div>
