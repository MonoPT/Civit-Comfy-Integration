//@ts-ignore
import {app} from "../../../scripts/app.js";
//@ts-ignore
import { api } from "../../../scripts/api.js";

import {ComfyApp} from '@comfyorg/comfyui-frontend-types'

const comfyApp: ComfyApp = app;

//@ts-ignore
import civiExplorerHtml from "./components/CivitMenu.html?raw"


app.extensionManager.registerSidebarTab({
  id: "CivitExplorer",
  icon: "pi pi-compass",
  title: "Civit",
  tooltip: "Use Civit.ai directly from Comfy",
  type: "custom",
  render: (el: HTMLElement) => {
    el.innerHTML = '<div id="CIVIT-EXPLORER-MARKER"></div>';
  }
});

let civiExplorer = document.createElement("div")
civiExplorer.innerHTML = civiExplorerHtml.replace("{{Server_address}}", "http://localhost:5173/")


let nav_selector = "nav.side-tool-bar-container"

// Waits for nav menu to load and add register mutation observer / adds civit explorer to DOM
function callFunctionUntilCondition() { 
  const intervalTime = 500; 
  let count = 0;
  let maxtTries = 100;

  const intervalId = setInterval(() => {
    count++; // Increment count for demonstration purposes
    
    let nav = window.document.querySelector(nav_selector);
    
    if (nav) {
      clearInterval(intervalId);  
      
      // Adds Civit Explorer Element to body
      document.querySelector("body")?.prepend(civiExplorer)
      
      // Forces side bar to close
      const config = { attributes: true, childList: true, subtree: true };
      const observer = new MutationObserver(callback);
      observer.observe(nav, config);
    }
    
    
    if (count >= maxtTries) { // Errors on max tries expired
      clearInterval(intervalId); 
      
      app.extensionManager.toast.add({
        severity: "error",
        summary: "Error",
        detail: "Could not start Civit Explorer",
        life: 5000
      });
      
    }
  }, intervalTime);
}



callFunctionUntilCondition();

// Handles opening Civit Explorer Menu
const callback = (mutationList: MutationRecord[], observer: MutationObserver) => {
  if (document.contains(document.querySelector("#CIVIT-EXPLORER-MARKER"))) {
    let civit_menu_btn: HTMLButtonElement = document.querySelector(`${nav_selector} .CivitExplorer-tab-button`)!;
    civit_menu_btn.click()
    
    let popup = document.querySelector("#Civit-POPUP")!;
    popup.classList.add("active");
  }
};

// Handles closing Civit Explorer Menu
window.addEventListener("CloseCivitExplorer", () => {
  let popup = document.querySelector("#Civit-POPUP")!;
  
  popup.classList.remove("active");
})

console.log(app.extensionManager.setting)