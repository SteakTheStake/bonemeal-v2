import { appWindow } from '@tauri-apps/api/window';

async function maxWin(){
    const maximized = await appWindow.isMaximized();
    if (maximized) {
        await appWindow.unmaximize();
    }else{
        await appWindow.maximize();
    }
}

async function hideWin(){
    await appWindow.minimize();
}

async function closeWin(){
    await appWindow.close();
}