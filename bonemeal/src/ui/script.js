import { appWindow } from '/@tauri-apps/api/window'
window.addEventListener("DOMContentLoaded", (event) => {
    const el1 = document.getElementById('titlebar-minimize');
    const el2 = document.getElementById('titlebar-maximize');
    const el3 = document.getElementById('titlebar-close');
    if (el1) {
    el1.addEventListener('click', () => appWindow.minimize())
    }
    if (el2) {
    el2.addEventListener('click', () => appWindow.toggleMaximize())
    }
    if (el3) {
    el3.addEventListener('click', () => appWindow.close())
    }
});