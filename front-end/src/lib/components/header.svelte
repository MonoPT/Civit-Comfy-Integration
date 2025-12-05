<script lang="ts">
    import Button from "./button.svelte"
    let {current_route = ""} = $props()
    import Separator from "$lib/components/form/separator.svelte"
    import Pill from "$lib/components/form/pill.svelte"
    import FilterManager from "$lib/api/filterManager"
    import Select from "$lib/components/form/select.svelte";
    
    import baseModels from "$lib/data/base_models"
    
    import { onMount } from "svelte";
    
    let openPopUp = $state(true)
    let filtersForm: HTMLElement;
        
    let formWrapper: HTMLFormElement
    
    let media_type = "Image"
    
    let base_models: {name: string, value: string}[] = $state([])
    
    const time_period_options = [
      {name: "Day", value: ""},
      {name: "Week", value: ""},
      {name: "Month", value: ""},
      {name: "Year", value: ""},
      {name: "All Time", value: ""},
    ]
    
    const sort_by = [
      {name: "Most Reactions", value: ""},
      {name: "Most Comments", value: ""},
      {name: "Newest", value: ""},
    ]
    
    const update_filters_data = () => {
      FilterManager.filters = {}
      const input = Array.from(formWrapper.querySelectorAll("input"))
    
      let keys = new Set(input.filter((i) => i.checked).map((i) => i.name).filter((i) => i.length > 0))
      
      keys.forEach((k) => {
        const values = Array.from(formWrapper.querySelectorAll(`input[name="${k}"]:checked`)).map((i: any) => i.value)
        
        FilterManager.update_filter(k, values)
      }) 
    }
    
    onMount(() => {
      update_filters_data()
      
      base_models = baseModels.filter((m) => m.type.toLowerCase() === media_type.toLowerCase()).sort((a,b) => a.name.localeCompare(b.name)).map((m) => {
        return {
          name: m.name,
          value: m.name
        }
      })
      
      formWrapper.addEventListener('input', update_filters_data)
            
      filtersForm.querySelector(".clear-button > *")?.addEventListener("click", () => {
        filtersForm.querySelectorAll(`input`).forEach((i) => i.checked = false)
        FilterManager.filters = {}
      })
    })
</script>
<form class="header" bind:this={formWrapper}>
    <Select  enable_search={false} default_selected="Day" singleOption maxContent empty_name="Time Period" icon={2} name="timePeriod" selection_name="Period" 
        options={time_period_options}/>
    
    <span></span>
    
    <div class="wrap">
        <Select  enable_search={false} default_selected="Most Reactions" maxContent singleOption empty_name="Sort by" icon={2} name="sort" selection_name="sort" 
            options={sort_by}/>
        
        <Select enable_search={false} maxContent empty_name="Media Type" default_selected={media_type} icon={2} name="mediaType" selection_name="Media" 
            options={[{name: "Image", value: "Image"}, {name: "Video", value: "Video"}]}/>
    </div>
    
    <div class="alignLeft">
        <div class="filters-wrapper">
            <Button onclick={() => openPopUp = !openPopUp} icon={4} active_route="_" current_route={current_route} no_bg={true} has_dropdown={true} dropdown_is_open={openPopUp}>Filters</Button>
            <div class="formContainer filters-popup" class:openPopUp bind:this={filtersForm}>                               
                <Separator>Base model</Separator>
                <div class="group">
                    <Select select_list_cards empty_name="Show all" icon={2} name="baseModel" selection_name="Base Models" 
                        options={base_models}/>
                </div>
                                
                <Separator>Modifiers</Separator>
                <div class="group">
                    <Button pill name="Metadata only" value="Metadata only" type="radioCheckbox" onclick={() => {}}  icon={0}  hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Metadata only</Button>
                    <Button pill name="Made On-site" value="Made On-site" type="radioCheckbox" onclick={() => {}}  icon={0} hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Made On-site</Button>
                    <Button pill name="Originals Only" value="Originals Only" type="radioCheckbox" onclick={() => {}}  icon={0} hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Originals Only</Button>
                    <Button pill name="Remixes Only" value="Remixes Only" type="radioCheckbox" onclick={() => {}}  icon={0} hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Remixes Only</Button>
                </div>
                
                <div class="clear-button">
                    <Button fullWidth icon={0} >Clear all filters</Button>
                </div>
            </div>
        </div>
    </div>
</form>

<style>
    .header {
        --gap: calc(var(--spacing) * 2);
        
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: var(--gap);
        padding-bottom: calc(var(--spacing) * 6);
        position: sticky;
        top: 0;
        padding-top: 1rem;
        z-index: 99999;
        background: var(--bgColor);
        
        .wrap {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: var(--gap);
        }
        
        .alignLeft {
            display: flex;
            justify-content: flex-end;
        }
        
        @media (max-width: 950px) {
            &, .wrap {
               grid-template-columns: 1fr; 
            }
            
            & > span {
                display: none;
            }
        }
    }
    
    
    .filters-wrapper {
        position: relative;
        --color: rgba(255,255,255, .6);
        
        .filters-popup {
            position: relative;
            padding: 1rem;
            min-width: 400px;
            width: 40vw;
            max-width: 600px;
            min-height: 100px;
            background: color-mix(in srgb, var(--bgColor) 98%, rgb(255,255,255) 2%);
            position: absolute;
            top: 100%;
            right: 0;
            z-index: 1;
            border: 1px solid rgba(255,255, 255, .2);
            border-radius: .4rem;
            opacity: 0;
            pointer-events: none;
            scale: .4;
            transform-origin: top right;
            transition: .12s;
            
            /*max-height: 80vh;
            overflow-y: auto;*/
            
            &.openPopUp {
                opacity: 1;
                pointer-events: all;
                scale: 1;
            }
            
            .group {
                display: flex;
                flex-wrap: wrap;
                gap: .75rem;
                padding-top: .85rem;
                padding-bottom: 2rem;
            }
            
        }
    }
</style>

<!-- 
<header>
    <div class="search-wrapper">
        <Select empty_name="Model Filter" icon={2} name="modelFilter" selection_name="Models" 
            options={[{name: "Model x", value: "x"}, {name: "Model y", value: "x"} ,{name: "Model z", value: "x"}]}/>
    </div>
    <nav>
        <ul class="typeSelector">
           <li><Button icon={1} active_route="/" current_route={current_route}>Images</Button></li>
           <li><Button icon={2} active_route="/models" current_route={current_route}>Models</Button></li>
           <li><Button icon={3} active_route="/downloads" current_route={current_route}>Downloads</Button></li>
        </ul>
        
    </nav>
</header>

<hr>

<style>
    header {
        min-height: 50px;
        display: flex;
        flex-direction: column;
        padding-block: .5rem;
    }
    
    
    
    nav {
        display: flex;
        justify-content: space-between;
    }
    
    .search-wrapper {
        min-height: 60px;
    }
    
    .typeSelector {
        display: flex;
        list-style: none;
        gap: .3rem;
    }
    
    hr {
        height: 1px;
        color: rgba(255,255,255, .2);
        display: block;
        position: relative;
        margin-block: 1.8rem;
        margin-top: .2rem;
    }
</style>
-->