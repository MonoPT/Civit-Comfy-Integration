<script lang="ts">
    import { onMount } from "svelte";

    let {
      icon = 0, 
      current_route = "", 
      active_route = "_", 
      no_bg = false, 
      has_dropdown = false,
      dropdown_is_open = false,
      fullWidth = undefined,
      onclick = () => {}
    } = $props()
    
    let is_active = $state(active_route === current_route)
    
    let buttonRef: HTMLElement;
    
    onMount(() => {
      buttonRef.addEventListener("click", () => onclick())
    })
</script>

<div class="button-wrapper" class:active={is_active} class:no_bg class:fullWidth bind:this={buttonRef} >
    {#if icon > 0}
        <div class="icon">
            {#if icon == 1}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 0 0 1.5-1.5V6a1.5 1.5 0 0 0-1.5-1.5H3.75A1.5 1.5 0 0 0 2.25 6v12a1.5 1.5 0 0 0 1.5 1.5Zm10.5-11.25h.008v.008h-.008V8.25Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
                </svg>
            {:else if icon == 2}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 16.875h3.375m0 0h3.375m-3.375 0V13.5m0 3.375v3.375M6 10.5h2.25a2.25 2.25 0 0 0 2.25-2.25V6a2.25 2.25 0 0 0-2.25-2.25H6A2.25 2.25 0 0 0 3.75 6v2.25A2.25 2.25 0 0 0 6 10.5Zm0 9.75h2.25A2.25 2.25 0 0 0 10.5 18v-2.25a2.25 2.25 0 0 0-2.25-2.25H6a2.25 2.25 0 0 0-2.25 2.25V18A2.25 2.25 0 0 0 6 20.25Zm9.75-9.75H18a2.25 2.25 0 0 0 2.25-2.25V6A2.25 2.25 0 0 0 18 3.75h-2.25A2.25 2.25 0 0 0 13.5 6v2.25a2.25 2.25 0 0 0 2.25 2.25Z" />
                </svg>
            {:else if icon == 3}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5M16.5 12 12 16.5m0 0L7.5 12m4.5 4.5V3" />
                </svg>
            {:else if icon == 4}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 3c2.755 0 5.455.232 8.083.678.533.09.917.556.917 1.096v1.044a2.25 2.25 0 0 1-.659 1.591l-5.432 5.432a2.25 2.25 0 0 0-.659 1.591v2.927a2.25 2.25 0 0 1-1.244 2.013L9.75 21v-6.568a2.25 2.25 0 0 0-.659-1.591L3.659 7.409A2.25 2.25 0 0 1 3 5.818V4.774c0-.54.384-1.006.917-1.096A48.32 48.32 0 0 1 12 3Z" />
                </svg>

            {/if}
        </div>
    {/if}
    
    <slot></slot>
    
      {#if has_dropdown}
          <div class="icon small dropdown" class:dropdown_is_open>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
            </svg>
          </div>
      {/if}
    
</div>

<style>
    .button-wrapper {
        user-select: none;
        padding: 1rem;
        padding-block: .3rem;
        padding-left: .5rem;
        cursor: pointer;
        
        border-radius: .13rem;
        transition: .1s;
        width: max-content;
        
        &.fullWidth {
            width: 100%;
            padding-block: .6rem;
        }
        
        &:not(.no_bg) {
            border: 1px solid rgba(255,255,255, .2);
            background: var(--bgLighter);
        }
        
        display: flex;
        align-items: center;
        justify-content: center;
        gap: .5rem;
        
        .icon {
            width: 20px;
            transition: .12s;
            transform-origin: center;
            
            &.small {
                width: 18px;
            }
            
            &.dropdown.dropdown_is_open {
                rotate: 180deg;
            }
        }
        
        &.active {
            background: #1971C2;
        }
        
        &:not(.active, .no_bg):hover {
            background: color-mix(in srgb, var(--bgLighter) 80%, black 20%);
            border-color: color-mix(in srgb, rgba(255,255,255, .2) 80%, black 20%);
        }
    }
</style>