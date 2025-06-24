<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte';
  
    let serverInfo: ServerInfo[] = [];
    let username:string = "";
    let password:string = "";
    interface ScreenSize {
        width: number;
        height: number;
    }
  
    interface ConnectRDPParams {
        is_sound: boolean;
        is_drives: boolean;
        is_printers: boolean;
        is_wallpaper: boolean;
        is_max_screen: boolean;
        screen_size: ScreenSize;
        tunnel_id: string;
        localport: number;
    }
  
    interface Server {
        access_url: string;
        company: string;
        group: string;
        name: string;
        tunnel_id: string;
    }
  
    interface ServerInfo {
        server: Server;
        rdp_param: ConnectRDPParams;
        is_connected: boolean;
        is_started_rdp: boolean;
    }
  
    async function fetchServerInfo() {
        serverInfo = await invoke('get_servers');
    }
  
    async function toggleConnection(tunnelid, isstarted, protocol) {
        await invoke('set_connect', { tunnelid, isstarted });
        await fetchServerInfo(); // Refresh server info after toggling
    }
  
    function handleCheckboxChange(event: Event, tunnelid: string, protocol: string) {
        const checkbox = event.target as HTMLInputElement;
        let isstarted = checkbox.checked;
        toggleConnection(tunnelid, isstarted, protocol);
    }
    async function singIn(tunnelid: string) {
      await invoke('start_rdp', { tunnelid, username, password });
      await fetchServerInfo();
    }
  
    function handleMaxScreenChange(event: Event) {
        const checkbox = event.target as HTMLInputElement;
        let isMaxScreen = checkbox.checked;
        // Если устанавливается максимальный экран, скрыть размеры экрана
        if (isMaxScreen) {
            // Здесь мы можем очистить или сделать что-то еще с размерами экрана
        }
    }
    async function updateSettings(tunnelid: string) {
        // Поиск информации о сервере для получения текущих настроек
      const server = serverInfo.find(info => info.server.tunnel_id === tunnelid);
      console.log("server", server);
  
      if (server) {
            // Обновление параметров подключения на основе формы
  
              server.rdp_param.screen_size.width = (document.getElementById('width') as HTMLInputElement).valueAsNumber;
              server.rdp_param.screen_size.height = (document.getElementById('height') as HTMLInputElement).valueAsNumber;
  
              await invoke('update_server_connect_params', {
                  tunnelid: server.rdp_param.tunnel_id,
                  issound: server.rdp_param.is_sound,
                  isdrives: server.rdp_param.is_drives,
                  isprinters: server.rdp_param.is_printers,
                  iswallpaper: server.rdp_param.is_wallpaper,
                  ismaxscreen: server.rdp_param.is_max_screen,
                  w: server.rdp_param.screen_size.width,
                  h: server.rdp_param.screen_size.height,
              });
              await fetchServerInfo(); // Обновление информации о серверах после изменений
        }
    }
  
    onMount(fetchServerInfo);
  </script>
  
  <div class="Container">
    <table class="table table-centered table-nowrap mb-0 rounded table-warning align-middle">
        <thead class="table-primary">
            <tr >
                <th class="border-0">Компания</th>
                <th class="border-0">Название сервера</th>
                <th class="border-0"></th>
                <th class="border-0"></th>
                <th class="border-0"></th>
            </tr>
        </thead>
        <tbody>
            {#each serverInfo as info}
            <tr class="{info.is_connected ? 'table-warning' : 'table-dark'}">
                <td>{info.server.company}</td>
                <td>{info.server.name}</td>
                <td>{info.rdp_param.localport}</td>
  
                <td>
                    <div class="form-check form-switch">
                        <input
                            class="form-check-input"
                            type="checkbox"
                            id="flexSwitchCheckDefault-{info.server.tunnel_id}"
                            checked={info.is_connected}
                            on:change={(e) => handleCheckboxChange(e, info.server.tunnel_id, info.server.protocol)}
                        >
                    </div>
                </td>
                <td>
                    <button class="btn btn-light" data-bs-toggle="modal" data-bs-target="#modalset{info.server.tunnel_id}">
                        <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="#5f6368">
                            <path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/>
                        </svg>
                    </button>
                  <!---modal button -->
                  <button class="btn btn-light" data-bs-toggle="modal" data-bs-target="#modal{info.server.tunnel_id}">
                      <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="#BB271A">
                          <path d="M800-600v-120H680v-80h120q33 0 56.5 23.5T880-720v120h-80Zm-720 0v-120q0-33 23.5-56.5T160-800h120v80H160v120H80Zm600 440v-80h120v-120h80v120q0 33-23.5 56.5T800-160H680Zm-520 0q-33 0-56.5-23.5T80-240v-120h80v120h120v80H160Zm80-160v-320h480v320H240Zm80-80h320v-160H320v160Zm0 0v-160 160Z"/>
                      </svg>
                  </button>
              </td>
            </tr>
            <!---modal in -->
            <div class="modal fade" id="modal{info.server.tunnel_id}" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel{info.server.tunnel_id}" aria-hidden="true">
              <div class="modal-dialog" style="background-color: #fff;">
                  <div class="modal-content">
                      <div class="modal-header">
                          
                          <h5 class="modal-title">сервер: {info.server.name}</h5>
                          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <br>
                        <div>
                        <small>компания: {info.server.company}</small>
                      </div>
                      <form  class="mt-4">
                          <!-- Form -->
                          <div class="form-group mb-4">
                            <label for="user">Имя пользователя</label>
                            <div class="input-group">
                              <span class="input-group-text" id="basic-addon1">
                                <svg class="icon icon-xs text-gray-600" fill="currentColor" viewBox="0 0 20 20"
                                  xmlns="http://www.w3.org/2000/svg">
                                  <path fill-rule="evenodd"
                                    d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0z"
                                    clip-rule="evenodd"></path>
                                </svg>
                              </span>
                              <input type="text" placeholder="username" name="user" class="form-control" bind:value={username}>
                            </div>
                          </div>
                          <!-- End of Form -->
                          <div class="form-group">
                            <div class="form-group mb-4">
                              <label for="password">Пароль</label>
                              <div class="input-group">
                                <span class="input-group-text" id="basic-addon2">
                                  <svg class="icon icon-xs text-gray-600" fill="currentColor" viewBox="0 0 20 20"
                                    xmlns="http://www.w3.org/2000/svg">
                                    <path fill-rule="evenodd"
                                      d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z"
                                      clip-rule="evenodd"></path>
                                  </svg>
                                </span>
                                <input type="password" name="username" class="form-control" bind:value={password}>
                              </div>
                            </div>
                          </div>
                          <div class="d-grid">
                              <button type="button" class="btn btn-gray-800" data-bs-dismiss="modal" on:click={() => singIn(info.server.tunnel_id)}>Войти</button>
                          </div>
                        </form>
                      
                      </div>
              </div>
          </div>
            <!---end modal in -->
            <!--modal settings-->
            <div class="modal fade modal-fullscreen" id="modalset{info.server.tunnel_id}" tabindex="-1" aria-labelledby="modalset{info.server.tunnel_id}" aria-hidden="true">
              <div class="modal-dialog modal-dialog" style="background-color: #fff;">
                  <div class="modal-content ">
                      <div class="modal-header">
                          <h5 class="modal-title">Настройки подключения </h5>
                          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                      </div>
                      <div>сервер: {info.server.name}</div>
                      <div class="modal-body">
                          <form on:submit|preventDefault={() => updateSettings(info.server.tunnel_id)}>
                              <div class="row">
                                  <div class="col-md-6">
                                      <div class="form-group mb-4">
                                          <div class="form-check">
                                              <label class="form-check-label">Передавать звук</label>
                                              <input type="checkbox" class="form-check-input" bind:checked={info.rdp_param.is_sound}>
                                          </div>
                                      </div>
                                      <div class="form-group mb-4">
                                          <div class="form-check">
                                              <label class="form-check-label">Диски</label>
                                              <input type="checkbox" class="form-check-input" bind:checked={info.rdp_param.is_drives}>
                                          </div>
                                      </div>
                                  </div>
                                  <div class="col-md-6">
                                      <div class="form-group mb-4">
                                          <div class="form-check">
                                              <label class="form-check-label">Принтеры</label>
                                              <input type="checkbox" class="form-check-input" bind:checked={info.rdp_param.is_printers}>
                                          </div>
                                      </div>
                                      <div class="form-group mb-4">
                                          <div class="form-check">
                                              <label class="form-check-label">Обои</label>
                                              <input type="checkbox" class="form-check-input" bind:checked={info.rdp_param.is_wallpaper}>
                                          </div>
                                      </div>
                                  </div>
                              </div>
                              <div class="form-group mb-4">
                                  <div class="form-check">
                                      <label class="form-check-label">Пользовательский экран</label>
                                      <input type="checkbox" class="form-check-input" bind:checked={info.rdp_param.is_max_screen} on:change={handleMaxScreenChange}>
                                  </div>
                              </div>
                              <div class="form-group mb-4">
                                  <div class="form-check">
                                      <label for="width">Ширина экрана :
                                          <input 
                                          type="number" 
                                          min="683" 
                                          max="3840" 
                                          bind:value={info.rdp_param.screen_size.width}>
                                          </label>
                                      <input 
                                          type="range" 
                                          id="width" 
                                          class="form-range" 
                                          min="683" 
                                          max="3840" 
                                          bind:value={info.rdp_param.screen_size.width} 
                                          disabled={info.rdp_param.is_max_screen ? true : false}>
                                  </div>
                              </div>
                              <div class="form-group mb-4">
                                  <div class="form-check">
                                      <label for="height">Высота экрана:   
                                          <input 
                                          type="number" 
                                          min="384" 
                                          max="2160"
                                          bind:value={info.rdp_param.screen_size.height}
                                          disabled={info.rdp_param.is_max_screen ? true : false}>    
                                      </label>
                                      <input 
                                          type="range" 
                                          id="height" 
                                          class="form-range" 
                                          min="384" 
                                          max="2160" 
                                          bind:value={info.rdp_param.screen_size.height}
                                          disabled={info.rdp_param.is_max_screen ? true : false}>
                                      
                                  </div>
                              </div>
                              <button type="submit" class="btn btn-primary">Сохранить</button>
                          </form>
                      </div>
                  </div>
              </div>
          </div>
          
          
            {/each}
        </tbody>
    </table>
  </div>
  











  <div>
    <svg width="118.8" height="50.4" viewBox="0 0 99 42" fill="none" xmlns="http://www.w3.org/2000/svg">
    <g filter="url(#filter0_d_207_289)">
    <path d="M4.59375 33V28.125L15.9844 17.5781C16.9531 16.6406 17.7656 15.7969 18.4219 15.0469C19.0885 14.2969 19.5938 13.5625 19.9375 12.8437C20.2813 12.1146 20.4531 11.3281 20.4531 10.4844C20.4531 9.54687 20.2396 8.73958 19.8125 8.0625C19.3854 7.375 18.8021 6.84896 18.0625 6.48437C17.3229 6.10937 16.4844 5.92187 15.5469 5.92187C14.5677 5.92187 13.7135 6.11979 12.9844 6.51562C12.2552 6.91146 11.6927 7.47917 11.2969 8.21875C10.901 8.95833 10.7031 9.83854 10.7031 10.8594H4.28125C4.28125 8.76562 4.75521 6.94792 5.70313 5.40625C6.65104 3.86458 7.97917 2.67187 9.6875 1.82812C11.3958 0.984374 13.3646 0.562499 15.5938 0.562499C17.8854 0.562499 19.8802 0.968749 21.5781 1.78125C23.2865 2.58333 24.6146 3.69792 25.5625 5.125C26.5104 6.55208 26.9844 8.1875 26.9844 10.0312C26.9844 11.2396 26.7448 12.4323 26.2656 13.6094C25.7969 14.7865 24.9583 16.0937 23.75 17.5312C22.5417 18.9583 20.8385 20.6719 18.6406 22.6719L13.9688 27.25V27.4687H27.4063V33H4.59375ZM53.7148 11.3438C53.4961 10.5833 53.1888 9.91146 52.793 9.32812C52.3971 8.73437 51.9128 8.23438 51.3398 7.82813C50.7773 7.41146 50.1315 7.09375 49.4023 6.875C48.6836 6.65625 47.8867 6.54688 47.0117 6.54688C45.3763 6.54688 43.9388 6.95312 42.6992 7.76562C41.4701 8.57812 40.5117 9.76042 39.8242 11.3125C39.1367 12.8542 38.793 14.7396 38.793 16.9687C38.793 19.1979 39.1315 21.0937 39.8086 22.6562C40.4857 24.2187 41.444 25.4115 42.6836 26.2344C43.9232 27.0469 45.3867 27.4531 47.0742 27.4531C48.6055 27.4531 49.9128 27.1823 50.9961 26.6406C52.0898 26.0885 52.9232 25.3125 53.4961 24.3125C54.0794 23.3125 54.3711 22.1302 54.3711 20.7656L55.7461 20.9688H47.4961V15.875H60.8867V19.9062C60.8867 22.7187 60.293 25.1354 59.1055 27.1562C57.918 29.1667 56.2826 30.7187 54.1992 31.8125C52.1159 32.8958 49.7305 33.4375 47.043 33.4375C44.043 33.4375 41.4076 32.776 39.1367 31.4531C36.8659 30.1198 35.0951 28.2292 33.8242 25.7812C32.5638 23.3229 31.9336 20.4062 31.9336 17.0312C31.9336 14.4375 32.3086 12.125 33.0586 10.0937C33.819 8.05208 34.8815 6.32292 36.2461 4.90625C37.6107 3.48958 39.1992 2.41146 41.0117 1.67187C42.8242 0.93229 44.7878 0.562499 46.9023 0.562499C48.7148 0.562499 50.4023 0.828124 51.9648 1.35937C53.5273 1.88021 54.9128 2.61979 56.1211 3.57812C57.3398 4.53646 58.3346 5.67708 59.1055 7C59.8763 8.3125 60.3711 9.76042 60.5898 11.3438H53.7148ZM94.2031 12.2031H87.3594C87.2344 11.3177 86.9792 10.5312 86.5938 9.84375C86.2083 9.14583 85.7135 8.55208 85.1094 8.0625C84.5052 7.57292 83.8073 7.19792 83.0156 6.9375C82.2344 6.67708 81.3854 6.54688 80.4688 6.54688C78.8125 6.54688 77.3698 6.95833 76.1406 7.78125C74.9115 8.59375 73.9583 9.78125 73.2813 11.3438C72.6042 12.8958 72.2656 14.7812 72.2656 17C72.2656 19.2812 72.6042 21.1979 73.2813 22.75C73.9688 24.3021 74.9271 25.474 76.1562 26.2656C77.3854 27.0573 78.8073 27.4531 80.4219 27.4531C81.3281 27.4531 82.1667 27.3333 82.9375 27.0938C83.7188 26.8542 84.4115 26.5052 85.0156 26.0469C85.6198 25.5781 86.1198 25.0104 86.5156 24.3438C86.9219 23.6771 87.2031 22.9167 87.3594 22.0625L94.2031 22.0937C94.026 23.5625 93.5833 24.9792 92.875 26.3438C92.1771 27.6979 91.2344 28.9115 90.0469 29.9844C88.8698 31.0469 87.4635 31.8906 85.8281 32.5156C84.2031 33.1302 82.3646 33.4375 80.3125 33.4375C77.4583 33.4375 74.9063 32.7917 72.6563 31.5C70.4167 30.2083 68.6458 28.3385 67.3437 25.8906C66.0521 23.4427 65.4063 20.4792 65.4063 17C65.4063 13.5104 66.0625 10.5417 67.375 8.09375C68.6875 5.64583 70.4688 3.78125 72.7188 2.5C74.9688 1.20833 77.5 0.562499 80.3125 0.562499C82.1667 0.562499 83.8854 0.822916 85.4688 1.34375C87.0625 1.86458 88.474 2.625 89.7031 3.625C90.9323 4.61458 91.9323 5.82812 92.7031 7.26562C93.4844 8.70312 93.9844 10.349 94.2031 12.2031Z" fill="white"/>
    </g>
    <defs>
    <filter id="filter0_d_207_289" x="0.28125" y="0.5625" width="97.9219" height="40.875" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
    <feFlood flood-opacity="0" result="BackgroundImageFix"/>
    <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0" result="hardAlpha"/>
    <feOffset dy="4"/>
    <feGaussianBlur stdDeviation="2"/>
    <feComposite in2="hardAlpha" operator="out"/>
    <feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.4 0"/>
    <feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow_207_289"/>
    <feBlend mode="normal" in="SourceGraphic" in2="effect1_dropShadow_207_289" result="shape"/>
    </filter>
    </defs>
    </svg>
</div>