

export interface User {
  refresh_token: string;
  access_token: string;
  exp: number;
  companys: ServerResponse[]; // Обновлено, если это соответствует структуре
  services: Record<string, Service>;
  is_save: boolean;
  name: string;
  email: string;
  company: string;
}
export interface Server {
    id: string;
    name: string;
    services: Service[];
}


export interface ScreenSize {
    width: number;
    height: number;
}

export interface ConnectRDPParams {
    is_sound: boolean;
    is_drives: boolean;
    is_printers: boolean;
    is_wallpaper: boolean;
    is_max_screen: boolean;
    screen_size: ScreenSize;
    tunnel_id: string;
    localport: number;
}


export interface ServerInfo {
    server: Server;
    rdp_param: ConnectRDPParams;
    is_connected: boolean;
    is_started_rdp: boolean;
}


export interface Service {
    id: string;
    protocol: string;
    port: number;
    access_url: string;
}



export interface ServerResponse {
    id: string;
    name: string;
    servers: Server[];
}

export interface ConnectRDPParams {
    is_sound: boolean;
    is_drives: boolean;
    is_printers: boolean;
    is_wallpaper: boolean;
    is_max_screen: boolean;
    screen_size: ScreenSize;
    tunnel_id: string;
    localport: number;
}
export interface ConnectSSHParams {
    tunnel_id: string;
    localport: number;
}

export interface ServiceStatus {
    rdp_param: ConnectRDPParams;
    ssh_param: ConnectSSHParams;
    is_connected: boolean;
    is_rdp: boolean;
    is_ssh: boolean;
}

export interface MainInfo {
    companys: ServerResponse[];
    service_status: Record<string, ServiceStatus>; // Key is service id
}

export interface ServiceInfo{
    service: Service;
    status: ServiceStatus;
    companyid: string;
}