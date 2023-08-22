import App from "./App.svelte"
import './index.scss';

const app = new App({
  target: document.getElementById("root") as HTMLElement,
})

export default app

console.log(process.env.DFX_NETWORK); // This will log either the
console.log(process.env.CANISTER_ID_INTERNET_IDENTITY); // This will log either the
