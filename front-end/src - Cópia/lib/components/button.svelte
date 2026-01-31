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
      onclick = () => {},
      hoverColor = "transparent",
      bgHover = false,
      extraPadding = false,
      name = "",
      value = "",
      type = "text",
      checked = false,
      pill = false,
      hideMarker = false
    } = $props()
        
    let initial_type = type
    
    let buttonRef: HTMLElement;
    
    onMount(() => {
      buttonRef.addEventListener("click", () => onclick())
      
      if(type === "radioCheckbox") {
        type = "checkbox"
      }
    })
</script>

<label class="button-wrapper  {initial_type === "radioCheckbox" ? 'radio' : ''}" class:pill class:active={active_route === current_route} class:extraPadding class:bgHover class:no_bg class:fullWidth bind:this={buttonRef} style="--bgHover: {hoverColor}">
    <input checked={checked} type="{type}" name="{name}" value="{value}" hidden style="position: absolute; top: 0; margin: 0; pointer-events: none;" >
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
            {:else if icon == 5}
                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M16 10L18.5768 8.45392C19.3699 7.97803 19.7665 7.74009 20.0928 7.77051C20.3773 7.79703 20.6369 7.944 20.806 8.17433C21 8.43848 21 8.90095 21 9.8259V14.1741C21 15.099 21 15.5615 20.806 15.8257C20.6369 16.056 20.3773 16.203 20.0928 16.2295C19.7665 16.2599 19.3699 16.022 18.5768 15.5461L16 14M6.2 18H12.8C13.9201 18 14.4802 18 14.908 17.782C15.2843 17.5903 15.5903 17.2843 15.782 16.908C16 16.4802 16 15.9201 16 14.8V9.2C16 8.0799 16 7.51984 15.782 7.09202C15.5903 6.71569 15.2843 6.40973 14.908 6.21799C14.4802 6 13.9201 6 12.8 6H6.2C5.0799 6 4.51984 6 4.09202 6.21799C3.71569 6.40973 3.40973 6.71569 3.21799 7.09202C3 7.51984 3 8.07989 3 9.2V14.8C3 15.9201 3 16.4802 3.21799 16.908C3.40973 17.2843 3.71569 17.5903 4.09202 17.782C4.51984 18 5.07989 18 6.2 18Z" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
            {:else if icon == 6}
                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M3 17L9 11L13 15L21 7M21 7V12M21 7H16" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
            {:else if icon == 7}
                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path fill-rule="evenodd" clip-rule="evenodd" d="M12 6.00019C10.2006 3.90317 7.19377 3.2551 4.93923 5.17534C2.68468 7.09558 2.36727 10.3061 4.13778 12.5772C5.60984 14.4654 10.0648 18.4479 11.5249 19.7369C11.6882 19.8811 11.7699 19.9532 11.8652 19.9815C11.9483 20.0062 12.0393 20.0062 12.1225 19.9815C12.2178 19.9532 12.2994 19.8811 12.4628 19.7369C13.9229 18.4479 18.3778 14.4654 19.8499 12.5772C21.6204 10.3061 21.3417 7.07538 19.0484 5.17534C16.7551 3.2753 13.7994 3.90317 12 6.00019Z" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
            {:else if icon == 8}
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-bookmark "><path d="M18 7v14l-6 -4l-6 4v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4z"></path></svg>
            {/if}
        </div>
    {/if}
    
    {#if type == "checkbox" && initial_type !== "radioCheckbox"}
        <div class="checkboxMarker" class:hideMarker></div>
    {/if}
    <slot></slot>
    {#if type == "radio" || initial_type === "radioCheckbox"}
        <div class="checkboxMarker radio" class:hideMarker></div>
    {/if}
    
    {#if has_dropdown}
        <div class="icon small dropdown" class:dropdown_is_open>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
            <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
        </svg>
        </div>
    {/if}
    
</label>

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
        
        &.pill {
            border-radius: 10000px !important;
            font-size: .9em;
            
            padding-inline: 1rem !important;
            padding-block: .45rem !important;
            
            &:not(:has(input:checked)) {
                .checkboxMarker {
                    position: absolute;
                    transform: scaleX(0);
                    --h: 0;
                }
            }
            
            &:has(input:checked) {
                &, &:hover {
                    background: #1971C2;
                }
            }
        }
        
        .checkboxMarker {
            --h: 1rem;
            width: var(--h);
            aspect-ratio: 1 / 1;
            background: var(--bgLighter);
            border-radius: .2rem;
            margin-right: calc(var(--spacing) * 1);
            position: relative;
            transition: .12s;
            
            &.radio {
                margin-right: 0;
                margin-left: auto;
                background: transparent;
            }
            
            &::after {
                content: '';
                line-height: var(--h);
                text-align: center;
                display: block;
                position: absolute;
                width: 100%;
                height: 100%;
                background: transparent;
                font-size: .8em;
                scale: 0;
                transition: .05s;
            }
        }
        
        &.fullWidth {
            width: 100%;
            padding-block: .6rem;
        }
        
        &:not(.no_bg) {
            background: var(--bgLighter);
        }
        
        display: flex;
        align-items: center;
        gap: .5rem;
        
        &.extraPadding {
            padding-block: calc(.25rem * 3);
            padding-inline: calc(.25rem * 4);
        }
        
        .icon {
            width: 20px;
            transition: .12s;
            transform-origin: center;
            
            display: flex;
            align-items: center;
            justify-content: center;
            
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
        
        &.bgHover{
            border-radius: .375rem;
            
            &:not(.active):hover {
                background: var(--bgHover);
            }
        }
        
        &:has(input:checked) {
            .checkboxMarker::after {
                content: '✔';
                scale: 1;
            }
        }
        
        &:has(input[type="radio"]:checked), &:global(.radioCheckbox) {
            background: color-mix(in srgb, var(--navBG) 94%, #fff 6%);
        }
        
        .checkboxMarker.hideMarker {
            display: none;
        }
    }
</style>