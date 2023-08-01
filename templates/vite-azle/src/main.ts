import './index.scss';
import { backend } from './declarations/backend';

const button = document.getElementById("button") as HTMLButtonElement;
const countElement = document.getElementById("count") as HTMLSpanElement;

let count : BigInt = 0n;

export const increment = async () => {
  count = await backend.inc();
  countElement.textContent = count.toString();
}

export const refresh = async () => {
  count = await backend.get();
  countElement.textContent = count.toString();
}

button.addEventListener('click', async () => {
  button.style.opacity = '0.5';
  await backend.inc();
  button.style.opacity = '1';
});


refresh();