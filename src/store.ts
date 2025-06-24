import { writable } from 'svelte/store';
import type { User, ServiceInfo } from './types';


export const user = writable<User | null>(null);
export let serverdata = writable<ServiceInfo  | null>(null);
export let activeTab = writable<string| null>("server");
