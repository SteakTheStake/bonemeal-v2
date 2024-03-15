import { appWindow } from '@tauri-apps/api/window';

document.addEventListener('DOMContentLoaded', function() {
    const minimizeButton = document.getElementById('titlebar-min');
    const maximizeButton = document.getElementById('titlebar-max');
    const closeButton = document.getElementById('titlebar-close');

    minimizeButton.addEventListener('click', async function() {
        try {
            await appWindow.minimize();
        } catch (error) {
            console.error('Failed to minimize window:', error);
        }
    });

    maximizeButton.addEventListener('click', async function() {
        try {
            const maximized = await appWindow.isMaximized();
            if (maximized) {
                await appWindow.unmaximize();
            } else {
                await appWindow.maximize();
            }
        } catch (error) {
            console.error('Failed to toggle window maximization:', error);
        }
    });

    closeButton.addEventListener('click', async function() {
        try {
            await appWindow.close();
        } catch (error) {
            console.error('Failed to close window:', error);
        }
    });
});