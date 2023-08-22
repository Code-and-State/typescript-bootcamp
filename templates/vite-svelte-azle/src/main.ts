import App from "./App.svelte"
import './index.scss';

const app = new App({
  target: document.getElementById("root") as HTMLElement,
})

export default app