const appWindow = window.window.getCurrent();

// Maximize the window
document.getElementById('titlebar-maximize').addEventListener('click', () => {
    appWindow.maximize();
});

// Minimize the window
document.getElementById('titlebar-minimize').addEventListener('click', () => {
    appWindow.minimize();
});

// Close the window
document.getElementById('titlebar-close').addEventListener('click', () => {
    appWindow.close();
});