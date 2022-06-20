import { appWindow } from '@tauri-apps/api/window';

const app = document.querySelector<HTMLDivElement>('#app')!

appWindow.setTitle("Dinovel!")

app.innerHTML = `
  <h1>Hello Dinovel!</h1>
`
