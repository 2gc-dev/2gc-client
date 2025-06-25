<script lang="ts">
    export let title: string = 'Уведомление';
    export let body: string = '';
    export let url: string = '';
    export let onClose: () => void = () => {};
  </script>
  
  <div class="modal-backdrop">
    <div class="modal-window">
      <div class="modal-header">
        <div class="modal-icon">
          <svg width="36" height="36" viewBox="0 0 24 24" fill="none">
            <path d="M12 2C13.1 2 14 2.9 14 4V5.17C16.39 6.17 18 8.42 18 11V16L20 18V19H4V18L6 16V11C6 8.42 7.61 6.17 10 5.17V4C10 2.9 10.9 2 12 2Z" fill="url(#grad)" />
            <path d="M12 22C13.1 22 14 21.1 14 20H10C10 21.1 10.9 22 12 22Z" fill="url(#grad)" />
            <defs>
              <linearGradient id="grad" x1="0" y1="0" x2="24" y2="24" gradientUnits="userSpaceOnUse">
                <stop stop-color="#4faaff"/>
                <stop offset="1" stop-color="#7f65ff"/>
              </linearGradient>
            </defs>
          </svg>
        </div>
        <h2 class="modal-title">{title}</h2>
      </div>
      <div class="modal-body">{body}</div>
  
      {#if url}
        <a class="modal-link" href={url} target="_blank">Подробнее</a>
      {/if}
  
      <button class="modal-button" on:click={onClose}>Понятно</button>
    </div>
  </div>
  
  <style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 3000;
    backdrop-filter: blur(6px);
  }
  
  .modal-window {
    background: #141416;
    border-radius: 20px;
    padding: 28px 36px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
    width: 100%;
    max-width: 420px;
    text-align: center;
    color: #fff;
    border: 1px solid rgba(255, 255, 255, 0.06);
  }
  
  .modal-header {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    margin-bottom: 20px;
  }
  
  .modal-icon {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .modal-title {
    font-size: 1.3rem;
    font-weight: 600;
    color: #eaeaea;
  }
  
  .modal-body {
    font-size: 1rem;
    color: #b0b0b0;
    margin-bottom: 20px;
  }
  
  .modal-link {
    display: inline-block;
    margin-bottom: 20px;
    color: #4faaff;
    text-decoration: underline;
    font-weight: 500;
    transition: color 0.2s;
  }
  
  .modal-link:hover {
    color: #7fcfff;
  }
  
  .modal-button {
    background: linear-gradient(135deg, #4faaff, #7f65ff);
    color: #fff;
    padding: 10px 28px;
    border: none;
    border-radius: 12px;
    font-size: 1rem;
    cursor: pointer;
    transition: opacity 0.2s;
  }
  
  .modal-button:hover {
    opacity: 0.9;
  }
  </style>
  