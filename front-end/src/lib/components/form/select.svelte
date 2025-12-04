<script lang="ts">
    import { onMount } from "svelte";
    import Button from '$lib/components/button.svelte';
    import Input from "$lib/components/form/input.svelte";
    
    type OptionItem = {
      name: string,
      value: string
    }
    
    type propList = {
      icon?: number,
      icon_pos?: "left" | "right",
      btn_onclick?: Function,
      empty_name?: string,
      name: string,
      selection_name: string,
      options: OptionItem[]
    }
    
    let {icon = 1, icon_pos = "right", btn_onclick = (a: any) => {}, empty_name = "", name = "", selection_name = "", options = []}: propList = $props()
    
    let btn: HTMLElement | undefined = $state();
    let input: HTMLInputElement
    let optionsContainer: HTMLFormElement
    let clearBTN: HTMLElement
    
    let component_ref: HTMLElement
    let menu_is_open = $state(false)
    let selected_items = $state(0)
    let selected_text = $derived(selected_items > 0 ? `${selected_items} ${selection_name}` : empty_name);
    let filtered_models = $derived(options.map((o) => o.name))
    
    onMount(() => {
      if(btn) {
        btn.addEventListener("click", () => btn_onclick()(input.value))
        
        input.addEventListener("keypress", (e) => {
          if (e.key == "Enter") {
            btn?.click()
          }
        })
      }  
                  
      optionsContainer.addEventListener("change", () => {
        let checkbox = optionsContainer.querySelectorAll("input[type='checkbox']:checked")
        selected_items = checkbox.length
      })
      
      clearBTN.addEventListener("click", () => {
        optionsContainer.querySelectorAll("input[type='checkbox']").forEach((el: any) => el.checked = false)
        selected_items = 0
      })
      
      window.addEventListener("click", (e: any) => {
        if (component_ref.contains(e.target)) return
        
        menu_is_open = false
      })
    })
    
    const update_filters = (val: string) => {
      filtered_models = options.filter(option => option.name.toLowerCase().includes(val)).map((o) => o.name);
    }
</script>

<div class="input-wrapper {icon > 0 ? 'icon' : ''} {icon_pos}" class:menuOpen={menu_is_open} bind:this={component_ref}>
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
        <input readonly type="text" placeholder="" value={selected_text} bind:this={input} onclick={() => menu_is_open = !menu_is_open}>
    </label>
    <form class="dropdown-container" bind:this={optionsContainer}>
        <div class="search-box">
            <div class="input-wrapper">
                <Input on_change={() => update_filters} button_text="" placeholder="Search..." icon={1} icon_pos="left" />
            </div>
            <div class="wrap">
                <span>{selected_items} items selected</span>
                <span class="selectable" bind:this={clearBTN}>Clear all</span>
            </div>
        </div>
        <div class="hr"></div>
        {#each options as option}
            <li style={filtered_models.includes(option.name) ? '' : 'display: none'}><Button name={name} value={option.value} type="checkbox" onclick={() => {}} fullWidth icon={0} no_bg={true} hoverColor="#3c3d42" bgHover={true} extraPadding={true}>{option.name}</Button></li> 
        {/each}
        
        {#if filtered_models.length < 1}
            <h2 class="noResults">No results found.</h2>
        {/if}
    </form>
</div>

<style>
    .hr {
        height: 1px;
        background-color: #494a50;
        margin-block: calc(var(--spacing)* 4);
    }
    
    .noResults {
        text-align: center;
        font-size: 1rem;
        font-weight: 300;
        padding-block: 1.1rem;
        opacity: .6;
    }
    
    .input-wrapper {
        .dropdown-container {
            opacity: 1;
            transform: scaleY(0);
            transition: .12s;
        }
        
        &.menuOpen {
            .dropdown-container {
                opacity: 1;
                transform: scaleY(1);
            }
            
            .wrap .icon {
                rotate: 90deg;
            }
        }
    }
    
    .dropdown-container {
        position: absolute;
        min-height: 10px;
        background: var(--navBG);
        border: 1px solid rgba(255,255,255, .1);
        padding: 1rem;
        width: 100%;
        position: absolute;
        top: calc(100% + var(--spacing) * 2);
        z-index: 999;
        border-radius: calc(var(--spacing) * 2);
        list-style: none;
        max-height: 500px;
        overflow-y: scroll;
        scrollbar-width: none;
        transform-origin: top center;
 
        .search-box {
            .input-wrapper {
                border: 1px solid #494a50;
                border-radius: .5rem;
            }
            
            .wrap {
                margin-top: calc(var(--spacing)* 2);
                
                span {
                    font-size: .875rem;
                    line-height: calc(1.25/.875);
                    
                    &.selectable {
                        cursor: pointer;
                        transition: .1s;
                        
                        &:hover {
                            text-decoration: underline;
                        }
                    }
                }
            }
        }
    }
    
    .input-wrapper {
        background-color: var(--bgLighter);
        
        border-radius: .5rem;
        box-sizing: border-box;
        position: relative;
        display: flex;
        align-items: center;
             
        input {
            grid-area: a;
            
            &[readonly] {
                user-select: none;
                
                &::selection {
                    background: rgba(255,255,255, .1);
                }
            }
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
            transition: .1s;
            
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