<script lang="ts">
    import { onMount } from "svelte";

    type propList = {
      icon?: number,
      icon_pos?: "left" | "right",
      button_text?: string,
      btn_onclick?: Function,
      placeholder?: string,
      on_change?: Function
    }
    
    let {icon = 0, icon_pos = "left", button_text = "Login", on_change = (a: any) => {}, btn_onclick = (a: any) => {}, placeholder = ""}: propList = $props()
    
    let btn: HTMLElement | undefined = $state();
    let input: HTMLInputElement
    
    onMount(() => {
      if(btn) {
        btn.addEventListener("click", () => btn_onclick()(input.value))
        
        input.addEventListener("keypress", (e) => {
          if (e.key == "Enter") {
            btn?.click()
          }
        })
      }
      
      input.addEventListener("input", () => {
        if (typeof on_change() == "function") {
          on_change()(input.value)
        }
      })
    })
</script>

<div class="input-wrapper {icon > 0 ? 'icon' : ''} {icon_pos}">
    <label class="wrap">
        {#if icon > 0}
            <div class="icon">
                <svg viewBox="0 -0.5 25 25" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path fill-rule="evenodd" clip-rule="evenodd" d="M5.5 10.7655C5.50003 8.01511 7.44296 5.64777 10.1405 5.1113C12.8381 4.57483 15.539 6.01866 16.5913 8.55977C17.6437 11.1009 16.7544 14.0315 14.4674 15.5593C12.1804 17.0871 9.13257 16.7866 7.188 14.8415C6.10716 13.7604 5.49998 12.2942 5.5 10.7655Z" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> <path d="M17.029 16.5295L19.5 19.0005" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
            </div>
        {/if}
        <input type="text" placeholder="{placeholder}" bind:this={input}>
    </label>
    
    {#if button_text.length > 0}
        <button bind:this={btn}>{button_text}</button>
    {/if}
</div>

<style>
    .input-wrapper {
        background-color: var(--bgLighter);
        
        border-radius: .5rem;
        box-sizing: border-box;
        
        display: flex;
        align-items: center;
        width: 100%;
        
        &:has(button) {
            display: grid;
            grid-template-columns: 1fr auto;
        }
        
        button {
            background: var(--bgButton);
            color: #fff;
            outline: none;
            border: none;
            padding-inline: calc(var(--spacing)  * 5);
            border-radius: inherit;
            border-top-left-radius: 0;
            border-bottom-left-radius: 0;
            cursor: pointer;
            font-size: 1em;
            transition: .12s;
            border-left: 2px solid var(--bgColor);
            height: 100%;
            
            &:hover {
                background-color: color-mix(in srgb, var(--bgButton) 90%, #fff 10%);
            }
        }
        
        input {
            grid-area: a;
        }
        
        .wrap {
            width: 100%;
            display: grid;
            grid-template-columns: auto 1fr;
            grid-template-areas: "b a";
            gap: calc(var(--spacing)  * 2);
            padding-inline: calc(var(--spacing) * 4);
            min-height: 40px;
            align-items: center;
        }
        
        .icon {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 1.8rem;
            aspect-ratio: 1 / 1;
            opacity: .7;
            grid-area: b;
        }
                
        &:has(.icon).right .wrap {
            grid-template-areas: "a b";
            grid-template-columns: 1fr auto;
        }
    }
    
    input {
        width: 100%;
        border: none;
        font-size: .875rem;
        background-color: #0000;
        outline: none;
        color: rgba(255,255,255, .7);
    }
</style>