<script lang="ts">
    import { onMount } from "svelte";
    
    let {open = false, is_mobile=false} = $props()
    
    let button: HTMLElement
    let buttonClose: HTMLElement
    
    onMount(() => {
      buttonClose.addEventListener("click", () => window.dispatchEvent(new CustomEvent("closeVisualizer")))
      
      
      
      if(!button) return
      button.addEventListener("click", () => window.dispatchEvent(new CustomEvent("visualizerToggleMeta")))
    })
</script>

<div class="wrapper" class:open class:is_mobile>
    <div class="icon" bind:this={buttonClose}>
        <svg viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M11.7816 4.03157C12.0062 3.80702 12.0062 3.44295 11.7816 3.2184C11.5571 2.99385 11.193 2.99385 10.9685 3.2184L7.50005 6.68682L4.03164 3.2184C3.80708 2.99385 3.44301 2.99385 3.21846 3.2184C2.99391 3.44295 2.99391 3.80702 3.21846 4.03157L6.68688 7.49999L3.21846 10.9684C2.99391 11.193 2.99391 11.557 3.21846 11.7816C3.44301 12.0061 3.80708 12.0061 4.03164 11.7816L7.50005 8.31316L10.9685 11.7816C11.193 12.0061 11.5571 12.0061 11.7816 11.7816C12.0062 11.557 12.0062 11.193 11.7816 10.9684L8.31322 7.49999L11.7816 4.03157Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
    </div>
    
    <div class="icon mobileOpenMeta" bind:this={button}>
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-chevron-up "><path d="M6 15l6 -6l6 6"></path></svg>
    </div>
</div>

<style>
    .wrapper {
        display: flex;
        align-items: center;
        height: 20px;
        
        &.open .mobileOpenMeta {
            rotate: 180deg;
        }
        
        &:not(.is_mobile) .mobileOpenMeta {
            display: none;
        }
        
        .icon {
            display: flex;
            align-items: center;
            justify-content: center;
            background: rgba(255,255,255, .1);
            padding: .5rem;
            border-radius: 100%;
            
            transform-origin: center;
            
            svg {
                width: 1rem;
                display: block;
                height: 50%;
                aspect-ratio: 1 / 1;
            }
            
            &.mobileOpenMeta {
                margin-left: auto;
            }
        }
    }
</style>