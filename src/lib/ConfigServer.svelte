<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { User, ScreenSize, Server, ServerInfo,   ServiceInfo, ConnectRDPParams } from '../types';
    import { user, serverdata, activeTab } from '../store';
    

    
    let username: string = "";
    let password: string = "";
    
    let server_data: ServiceInfo = $serverdata;
    let data = server_data.status.rdp_param;
    let allowed_size = [
        "Использовать мой размер",
        "640 x 480 (VGA)",
        "800 x 600 (SVGA)",
        "1024 x 768 (XGA)",
        "1280 x 720 (HD, 720p)",
        "1280 x 800 (WXGA)",
        "1366 x 768 (HD)",
        "1440 x 900 (WXGA+)",
        "1600 x 900 (HD+)",
        "1680 x 1050 (WSXGA+)",
        "1920 x 1080 (Full HD, 1080p)",
        "1920 x 1200 (WUXGA)",
        "2560 x 1440 (QHD, 1440p)"
    ];
    let size_data = [[0,0],[640, 480], [800, 600], [1024,768], [1280, 800], [1366, 768], [1440, 900], [1600, 900], [1680, 1050], [1920, 1200], [2560, 1440]]
    
    let selectedScreenSize: string = findAllowedSize(server_data.status.rdp_param.screen_size.width, server_data.status.rdp_param.screen_size.height);
    function findAllowedSize(width: number, height: number): string {
        // Находим индекс в массиве size_data
        let index = size_data.findIndex(size => size[0] === width && size[1] === height);
        
        // Если размеры найдены, возвращаем соответствующую строку из allowed_size
        if (index !== -1) {
            return allowed_size[index];
        } else {
            return allowed_size[0]; // Если размеры не найдены, возвращаем первый элемент ("Использовать мой размер")
        }
    }

    
    function parseScreenSize(size: string) {
      let index = allowed_size.indexOf(size);
      return size_data[index]

    }

    async function updateSettings() {
        let w = 0;
        let h = 0;
        let ismaxscreen = false;

        if (selectedScreenSize === allowed_size[0]) {
            ismaxscreen = true;
        } else {
            let parse_data = parseScreenSize(selectedScreenSize);
            w = parse_data[0];
            h = parse_data[1];
        }

        await invoke('update_server_connect_params', {
            tunnelid: server_data.service.id,
            issound: data.is_sound,
            isdrives: data.is_drives,
            isprinters: data.is_printers,
            iswallpaper: data.is_wallpaper,
            ismaxscreen: ismaxscreen,  // Используем значение ismaxscreen
            w: w,                      // Обновляем ширину экрана
            h: h,                     // Обновляем высоту экрана
        });
        activeTab.set('loginserver');
    }

</script>



<section class="auth-form">
  <br><br>
    <div class="">
      
      {#if $serverdata.service.protocol == 'rdp'}
      <form class="">
        <div class="custom-data-2">
            <input type="checkbox" class="custom-checkbox" bind:checked={data.is_drives}>
            Перенаправление дисков
        </div>
        <div class="custom-data-2">
            <input type="checkbox" class="custom-checkbox" bind:checked={data.is_printers}>
            Перенаправление принтера
        </div>
        <div class="custom-data-2">
            <input type="checkbox" class="custom-checkbox" bind:checked={data.is_wallpaper}>
            Обои удаленного компьютера
        </div>
        <div class="custom-data-2">
            <input type="checkbox" class="custom-checkbox" bind:checked={data.is_sound}>
            Перенаправление звука
        </div>

        <!-- Dropdown Menu for Screen Size -->
        <div class="custom-data-2">Выбор разрешения экрана</div>
        
        <div class="">
            
            <select id="screen-size" bind:value={selectedScreenSize} class="custom-select">
                {#each allowed_size as size}
                    <option value={size}>{size}</option>
                {/each}
            </select>
            
        </div>

        <div class="custom-form-2">
            <button type="submit" class="custom-button" on:click="{updateSettings}">Сохранить</button>
            <button type="submit" class="custom-button" on:click="{updateSettings}">Не Сохранять</button>
        </div>
      </form>
      {:else}
      <p>Вы уже подключились к этому серверу</p>
      {/if}
    </div>
</section>
  
  <style>

.custom-select{
  margin-top: 15px;
  margin-left: 50px;
  height: 39px;
  width: 275px;
}
    .custom-form-2 {
      margin-top: 2px;
        display: flex;
        flex-direction: row; /* Размещаем элементы по горизонтали */
        justify-content: center; /* Выравниваем кнопки по центру */
        gap: 20px; /* Расстояние между кнопками */
    }
    .custom-form {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 60px;
    }
  
    .custom-input {
      width: 304px;
      height: 47px;
      border-radius: 5px;
    }
  
    .custom-button {
      width: 130px;
      height: 39px;
      border-radius: 10px;
      background:  radial-gradient(94.3% 185.81% at 94.3% 50%, #6868DD 0%, #5773DB 52%, #7B5CE0 100%);
      margin-top: 10px;
       /* Сдвигаем кнопку вправо */
       /* Добавляем внутренние отступы */
    }
  
    .custom-button2 {
      background-color: transparent;
      width: 37px;
      height: 37px;
      margin-left: auto;
      /* Отступ справа от кнопки */
    }
  
    .custom-checkbox {
      width: 20px;
      height: 20px;
      accent-color: #8d9194;
      border-radius: 10px;
    }
  
    
    .custom-data-2 {
      display: flex;
      
      color: white;

      margin-top: 11px;
      margin-left: 60px;
      width: 235px;
    }
  </style>