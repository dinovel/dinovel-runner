import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';

appWindow.setTitle("Dinovel!")

setTimeout(async () => {
  window.location.href = await invoke('server_address');
}, 0);
