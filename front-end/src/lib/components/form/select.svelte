<script lang="ts">
    import { onMount } from "svelte";

    type propList = {
      icon?: number,
      icon_pos?: "left" | "right",
      btn_onclick?: Function,
      value?: string
    }
    
    let {icon = 1, icon_pos = "right", btn_onclick = (a: any) => {}, value = ""}: propList = $props()
    
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
      
      
    })
</script>

<div class="input-wrapper {icon > 0 ? 'icon' : ''} {icon_pos}">
    <label class="wrap">
        {#if icon > 0}
            <div class="icon">
                {#if icon == 1}
                    <svg viewBox="0 -0.5 25 25" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path fill-rule="evenodd" clip-rule="evenodd" d="M5.5 10.7655C5.50003 8.01511 7.44296 5.64777 10.1405 5.1113C12.8381 4.57483 15.539 6.01866 16.5913 8.55977C17.6437 11.1009 16.7544 14.0315 14.4674 15.5593C12.1804 17.0871 9.13257 16.7866 7.188 14.8415C6.10716 13.7604 5.49998 12.2942 5.5 10.7655Z" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> <path d="M17.029 16.5295L19.5 19.0005" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
                {:else if icon == 2}
                    <div class="icon small dropdown">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                    </svg>
                    </div>
                {/if}
            </div>
        {/if}
        <input readonly type="text" placeholder="" value={value} bind:this={input}>
    </label>
    
    
</div>

<style>
    .input-wrapper {
        background-color: var(--bgLighter);
        
        border-radius: .5rem;
        box-sizing: border-box;
        
        display: flex;
        align-items: center;
             
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
            transform-origin: center;
            
            &.small {
                width: 1.4rem;
            }
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