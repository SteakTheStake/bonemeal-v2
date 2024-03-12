import { trace, info, error, attachConsole } from "tauri-plugin-log-api";

// with LogTarget::Webview enabled this function will print logs to the browser console
const detach = await attachConsole();

trace("Trace");
info("Info");
error("Error");

// detach the browser console from the log stream
detach();