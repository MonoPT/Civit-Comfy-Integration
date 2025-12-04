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
      maxContent?: boolean,
      singleOption?: boolean,
      options: OptionItem[],
      enable_search?: boolean,
      default_selected?: string,
      select_list_cards?: boolean
    }
    
    let {icon = 1, icon_pos = "right", btn_onclick = (a: any) => {}, empty_name = "", name = "", selection_name = "", options = [], maxContent = undefined, singleOption = false, enable_search = true, default_selected = "", select_list_cards = false}: propList = $props()
    
    let btn: HTMLElement | undefined = $state();
    let input: HTMLInputElement
    let optionsContainer: HTMLElement
    let clearBTN: HTMLElement | null = $state(null)
    let label: HTMLElement
    
    let minimMargin = 16
    let absMaxPopupSelect = 500
    let maxHeightSelectPopup = $state(500)
    
    let component_ref: HTMLElement
    let menu_is_open = $state(false)
    let selected_items = $state(0)
    let selected_items_names: string[] = $state([])
    let selected_text = $derived(
      selected_items > 1 ? `${selected_items} ${selection_name}` : empty_name
    );
    let filtered_items = $derived(options.map((o) => o.name))
    
    let original_empty_name = empty_name
    
    const update_selection = () => {
      let checkbox = Array.from(optionsContainer.querySelectorAll("input:checked")) as HTMLInputElement[]
      selected_items_names  = checkbox.map((el) => el.value)
      selected_items = checkbox.length
                     
      if (singleOption || checkbox.length === 1) {
        if (checkbox.length > 0) {
          empty_name = (checkbox[0] as HTMLInputElement).value
        } else {
          empty_name = original_empty_name
        }
      } else {
        empty_name = original_empty_name
      }
    }
    
    const update_filters = (val: string) => {
      filtered_items = options.filter(option => option.name.toLowerCase().includes(val)).map((o) => o.name);
    }
    
    const fixContainerSize = () => {
      if (!label) return
      
      const rect = label.getBoundingClientRect();
      
      maxHeightSelectPopup = absMaxPopupSelect
      let val = window.innerHeight - (rect.top + rect.height) - minimMargin;
      
      if (val < absMaxPopupSelect) {
        maxHeightSelectPopup = val
      }
    }
    
    onMount(() => {
      if(btn) {
        btn.addEventListener("click", (e) => {
          btn_onclick()(input.value)
        })
        
        input.addEventListener("keypress", (e) => {
          if (e.key == "Enter") {
            btn?.click()
          }
        })
      }  
              
      optionsContainer.addEventListener("change", () => {
        update_selection()
      })
      
      if (clearBTN) {
        clearBTN.addEventListener("click", () => {
          optionsContainer.querySelectorAll("input[type='checkbox']").forEach((el: any) => el.checked = false)
          selected_items = 0
          selected_items_names = []
        })
      }
      
      window.addEventListener("click", (e: any) => {
        if (component_ref && component_ref.contains(e.target)) return
        
        menu_is_open = false
      });
      
      label.addEventListener("click", (e) => { // Prevents popup window overflow
        const target = e.target as HTMLElement
        
        if (target.tagName !== "LABEL") return
        fixContainerSize()
      })
      
      const resizeObserver = new ResizeObserver(entries => {
        for (let entry of entries) {
          fixContainerSize()
        }
      });
      
      // Start observing the element
      resizeObserver.observe(label);
      
      update_selection()
    })
</script>
<div style="--popupMaxH: {maxHeightSelectPopup}" class:MaxContent={maxContent} class="input-wrapper mainContainer {icon > 0 ? 'icon' : ''} {icon_pos}" class:menuOpen={menu_is_open} bind:this={component_ref}>
    <label class="wrap" bind:this={label}>
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
        {#if select_list_cards && selected_items_names.length < 14 && selected_items_names.length > 0}
            <div class="selected-cards">
                {#each selected_items_names as item}
                    <span>{item}</span>
                {/each}
            </div>
        {/if}
        <!--selected_items_names.length > 1 /*(select_list_cards && selected_items_names.length < 14)*/-->
        <input hidden={selected_items_names.length < 1 ? false : (select_list_cards && selected_items_names.length < 14) } readonly type="text" placeholder="" value={selected_text} bind:this={input} onclick={() => menu_is_open = !menu_is_open}>
    </label>
    <div class="dropdown-container" bind:this={optionsContainer}>
        {#if enable_search}
            <div class="search-container">
                <div class="search-box">
                    <div class="input-wrapper">
                        <Input on_change={() => update_filters} button_text="" placeholder="Search..." icon={1} icon_pos="left" />
                    </div>
                    {#if !singleOption && selected_items_names.length > 0}
                        <div class="wrap">
                            <span>{selected_items} items selected</span>
                            <span class="selectable" bind:this={clearBTN}>Clear all</span>
                        </div>
                    {/if}
                </div>
                <div class="hr-container"><div class="hr"></div></div>
            </div>
        {/if}
        {#each options as option}
            <li style={filtered_items.includes(option.name) ? '' : 'display: none'}><Button checked={default_selected.toLowerCase() === option.name.toLowerCase()} name={name} value={option.value.length > 0 ? option.value : option.name} type={singleOption ? 'radio' : 'checkbox'} onclick={() => {}} fullWidth icon={0} no_bg={true} hoverColor="#3c3d42" bgHover={true} extraPadding={true}>{option.name}</Button></li> 
        {/each}
        
        {#if filtered_items.length < 1}
            <h2 class="noResults">No results found.</h2>
        {/if}
    </div>
</div>

<style>
    .hr-container {
        padding-block: calc(var(--spacing)* 4);
        .hr {
            height: 1px;
            background-color: #494a50;  
        }
    }
    
    .selected-cards {
        padding-block: .8rem;
        display: flex;
        flex-wrap: wrap;
        gap: .2rem;
        
        span {
            background: var(--navBG);
            padding-inline: .6rem;
            padding-block: .2rem;
            border-radius: .2rem;
            color: rgba(255,255,255, .7);
            font-weight: 300;
            font-size: .78em;
        }
    }
    
    .search-container {
        position: relative;
        position: sticky;
        top: 0;
        margin: 0;
        background: var(--navBG);
        z-index: 2;
    }
    
    .noResults {
        text-align: center;
        font-size: 1rem;
        font-weight: 300;
        padding-block: 1.1rem;
        opacity: .6;
    }
    
    .mainContainer {
        position: relative;
        
        &:has(.search-container) .dropdown-container {
            padding-top: 0 !important;
            
            .search-container {
                padding-top: 1rem;
            }
        }
    }
    
    label {
        * {
            pointer-events: none;
        }
        cursor: pointer;         
    }
    
    .mainContainer {
        width: 100%;
        
        &.MaxContent {
            min-width: 320px;
            width: max-content;
        }   
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
        top: calc(100% + 2px);
        z-index: 999;
        border-radius: calc(var(--spacing) * 2);
        list-style: none;
        max-height: calc(var(--popupMaxH) * 1px);
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