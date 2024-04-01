import { appWindow } from '@tauri-apps/api/window';

// Maximize the window
document.getElementById('titlebar-max')?.addEventListener('click', () => {
    appWindow.maximize();
});

// Minimize the window
document.getElementById('titlebar-min')?.addEventListener('click', () => {
    appWindow.minimize();
});

// Close the window
document.getElementById('titlebar-close')?.addEventListener('click', () => {
    appWindow.close();
});