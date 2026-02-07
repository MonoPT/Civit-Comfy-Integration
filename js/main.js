import { app as i } from "../../../scripts/app.js";
const l = /*html*/`<div id="Civit-POPUP">
    <div class="container p-dialog">
        <button
            onclick="window.dispatchEvent(new CustomEvent('CloseCivitExplorer'))"
            type="button"
            data-pc-name="button"
            data-p-disabled="false"
            class="flex items-center justify-center shrink-0 outline-hidden rounded-lg cursor-pointer disabled:opacity-50 disabled:pointer-events-none p-0 size-10 text-sm bg-white border-none text-neutral-950 dark-theme:bg-zinc-700 dark-theme:text-white absolute top-4 right-6 z-10 transition-opacity duration-200"
            pc66=""
            data-pc-section="root"
        >
            <i data-v-76dd6fc0="" class="pi pi-times text-sm"></i>
        </button>
        <iframe src="{{Server_address}}"></iframe>
    </div>
</div>

<style>
    #Civit-POPUP > .container {
        width: 100%;
        max-width: 90vw;
        height: 100%;
        max-height: 85vh;
        border-radius: var(--radius-2xl);
        box-sizing: border-box;
        transition: inherit;
        overflow: hidden;
    }

    #Civit-POPUP {
        background: var(--p-mask-background);
        position: fixed;
        top: 0;
        left: 0;
        padding: 2rem;
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        z-index: 99999999;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 0.12s;

        &:not(.active) {
            pointer-events: none;
            opacity: 0;

            & > .container {
                scale: 0.4;
                transform-origin: center;
            }
        }
    }

    #closeCivitExplorer > * {
        pointer-events: none;
    }
    
    iframe {
        width: 100%;
        height: 100%;
        border: none;
        background: transparent;
    }
</style>
`;

// Create extension button
i.extensionManager.registerSidebarTab({
  id: "CivitExplorer",
  icon: "pi pi-compass",
  title: "Civit",
  tooltip: "Use Civit.ai directly from Comfy",
  type: "custom",
  render: (e) => {
    e.innerHTML = '<div id="CIVIT-EXPLORER-MARKER"></div>';
  }
});

// Populate template
let s = document.createElement("div");
s.innerHTML = l.replace("{{Server_address}}", "/civit/?v=1.0");


/// Wait until it can mount the app
const d = "nav.side-tool-bar-container";

(function () {
  let n = 0, r = 100;
  const t = setInterval(() => {
    var a;
    n++;
    let o = window.document.querySelector(d);
    if (o) {
      clearInterval(t), (a = document.querySelector("body")) == null || a.prepend(s);
      const c = { attributes: !0, childList: !0, subtree: !0 };
      new MutationObserver(p).observe(o, c);
    }
    n >= r && (clearInterval(t), i.extensionManager.toast.add({
      severity: "error",
      summary: "Error",
      detail: "Could not start Civit Explorer",
      life: 5e3
    }));
  }, 500);
})()


const p = (e, n) => {
  document.contains(document.querySelector("#CIVIT-EXPLORER-MARKER")) && (document.querySelector(`${d} .CivitExplorer-tab-button`).click(), document.querySelector("#Civit-POPUP").classList.add("active"));
};

// Close button
window.addEventListener("CloseCivitExplorer", () => {
  document.querySelector("#Civit-POPUP").classList.remove("active");
});
//console.log(i.extensionManager.setting);
