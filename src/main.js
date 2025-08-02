document.addEventListener("DOMContentLoaded", () => {
  // Controlla periodicamente le notifiche
  setInterval(() => {
    const hasUnread = document.querySelector('[aria-label*="unread"]') !== null;
    if (window.__TAURI__) {
      window.__TAURI_INVOKE__("update_tray_icon", {
        hasNotifications: hasUnread,
      });
    }
  }, 5000);
});
