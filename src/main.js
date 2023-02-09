const { invoke } = window.__TAURI__.tauri;
const { dialog } = window.__TAURI__;

let path;

window.addEventListener("DOMContentLoaded", () => {
  document.getElementById('select-save-location').addEventListener('click', async () => {
    path = await dialog.open({
      directory: true,
      multiple: false,
      title: 'Choose download location',
    });
    document.getElementById('save-location').textContent = path;
  });

  document.forms.downloadForm.addEventListener('submit', async (e) => {
    const url = new FormData(e.target).get('url');
    if (!url || !path) {
      return;
    }
    e.preventDefault();
    const success = await invoke('download_video', {
      url,
      path,
    });
    document.getElementById('success').textContent = success;
  });
});
