function appendToConsoleLog(level, message) {
    const consoleLog = document.getElementById('console-log');
    const timestamp = new Date().toLocaleString();
    const logMessage = `${timestamp} [${level}] ${message}`;

    const logEntry = document.createElement('div');
    logEntry.classList.add('log-entry');

    const timestampSpan = document.createElement('span');
    timestampSpan.classList.add('timestamp');
    timestampSpan.textContent = timestamp;

    const levelSpan = document.createElement('span');
    levelSpan.classList.add('level');
    levelSpan.textContent = `[${level}]`;

    const messageSpan = document.createElement('span');
    messageSpan.classList.add('message');
    messageSpan.textContent = message;

    logEntry.appendChild(timestampSpan);
    logEntry.appendChild(document.createTextNode(' '));
    logEntry.appendChild(levelSpan);
    logEntry.appendChild(document.createTextNode(' '));
    logEntry.appendChild(messageSpan);

    consoleLog.appendChild(logEntry);
    consoleLog.scrollTop = consoleLog.scrollHeight;
}