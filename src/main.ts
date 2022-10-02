import 'bootstrap/dist/css/bootstrap.min.css';
import App from './App.svelte';
import { registerSW } from 'virtual:pwa-register';

const updateSW = registerSW({
  onNeedRefresh() {
    if ( showUpdatePrompt() ) {
      updateSW();
    }
  },
  onOfflineReady() {
    alert("Se encuentra navegando UpVent sin conexión.");
  },
});

function showUpdatePrompt(): boolean {
  return confirm("Hay una nueva versión de UpVent disponible\
   ¿Desea recargar la página para disfrutar del nuevo contenido?");
}

const app: App = new App({
  target: document.body,
  intro: true
});

export default app;