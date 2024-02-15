const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  console.log("Inside greet function"); // Add console log here
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

let primeInputEl;
let primeMsgEl;
let primeTimeEl;
let primeLargestEl;

async function prime() {
  console.log("Inside prime function"); // Add console log here
  const number = primeInputEl.value;
  const result = await invoke('prime', number);
  
  // update your GUI with the result
  primeTimeEl.textContent = result[0];
  primeMsgEl.textContent = result[1];
  primeLargestEl.textContent = result[2];
}

window.addEventListener("DOMContentLoaded", () => {
  primeInputEl = document.querySelector("#prime-input");
  primeMsgEl = document.querySelector("#prime-msg");
  primeTimeEl = document.querySelector("#prime-time");
  primeLargestEl = document.querySelector("#prime-largest");
  
  document.querySelector("#prime-form").addEventListener("submit", (e) => {
    e.preventDefault();
    console.log("Inside prime form submit event listener"); // Add console log here
    prime();
  });
});
