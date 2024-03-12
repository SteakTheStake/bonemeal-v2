import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';


// Define an interface for the routes object
interface Routes {
  '/': string;
  '/tools': string;
  '/stats': string;
  '/log': string;
  '/settings': string;
}

// Define the routes and their corresponding template files
const routes: Routes = {
  '/': '../ui/index.html',
  '/tools': '../ui/tools.html',
  '/stats': '../ui/stats.html',
  '/log': '../ui/log.html',
  '/settings': '../ui/settings.html',
};

// Get the current year for the copyright notice
const yearElement = document.getElementById('year');
if (yearElement) {
  yearElement.textContent = new Date().getFullYear().toString();
}

// Minimize the window when the minimize button is clicked
const minimizeButton = document.getElementById('titlebar-minimize');
if (minimizeButton) {
  minimizeButton.addEventListener('click', () => {
    appWindow.minimize();
  });
}

// Maximize the window when the maximize button is clicked
const maximizeButton = document.getElementById('titlebar-maximize');
if (maximizeButton) {
  maximizeButton.addEventListener('click', () => {
    appWindow.toggleMaximize();
  });
}

// Close the window when the close button is clicked
const closeButton = document.getElementById('titlebar-close');
if (closeButton) {
  closeButton.addEventListener('click', () => {
    appWindow.close();
  });
}

// Function to load the content of a template file into the main container
async function loadTemplate(route: keyof Routes) {
  const container = document.querySelector('.container.main');
  if (container) {
    const response = await invoke('render_template', { template: routes[route] });
    container.innerHTML = response as string;
  }
}

// Listen for navigation events and load the corresponding template
window.addEventListener('navigate/', () => loadTemplate('/'));
window.addEventListener('navigate/tools', () => loadTemplate('/tools'));
window.addEventListener('navigate/stats', () => loadTemplate('/stats'));
window.addEventListener('navigate/log', () => loadTemplate('/log'));
window.addEventListener('navigate/settings', () => loadTemplate('/settings'));

// Load the initial template
loadTemplate('/');