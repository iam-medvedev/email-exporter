const invoke = window.__TAURI__.invoke;

const form = document.getElementById("form");

form.addEventListener("submit", (e) => {
  e.preventDefault();

  invoke("run_email_export", {});
});
