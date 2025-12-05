<script lang="ts">
    import Button from "./button.svelte"
    import Separator from "$lib/components/form/separator.svelte"
    import FilterManager from "$lib/api/filterManager"
    import Select from "$lib/components/form/select.svelte";
    
    import baseModels from "$lib/data/base_models"
    import techniquesList from "$lib/data/techniques"
    
    import { onMount } from "svelte";
    
    let {media, current_route = "", page_title} = $props()
    
    let filtersForm: HTMLElement;
        
    let formWrapper: HTMLFormElement
        
    let media_type = $state((media.split(",") as string[]).map((s) => s.toLowerCase()))
       
    const update_base_models = () => {
      return baseModels.filter((m) => media_type.includes(m.type.toLowerCase())).sort((a,b) => a.name.localeCompare(b.name)).map((m) => {
        return {
          name: m.name,
          value: m.name
        }
      })
    }
    
    let base_models: {name: string, value: string}[] = $state(update_base_models())
    
    let techniques: {name: string, value: string}[] = $state(techniquesList)
    
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
      {name: "Most Collected", value: ""},
      {name: "Newest", value: ""},
      {name: "Oldest", value: ""},
    ]
    
    const update_filters_data = (e: Event | null) => {
      media_type = (Array.from(formWrapper.querySelectorAll(`input[name="mediaType"]:checked`)) as HTMLInputElement[]).map((i) => i.value)
            
      if (e) { // if its searching field skip
        //@ts-ignore
        const name = e.target.name.trim()
        if(name === "openPopUpButton") return
        if (name.length < 1) return // will skip fetch if input name is null
        //@ts-ignore
        if(e.target.closest(".search-container")) return
      }
      
      FilterManager.filters = {}
      const input = Array.from(formWrapper.querySelectorAll("input"))
    
      let keys = new Set(input.filter((i) => i.checked).map((i) => i.name).filter((i) => i.length > 0))
      
      keys.forEach((k) => {
        const values = Array.from(formWrapper.querySelectorAll(`input[name="${k}"]:checked`)).map((i: any) => i.value)
        
        FilterManager.update_filter(k, values)
      }) 
    }
        
    onMount(() => {  
      update_filters_data(null)
            
      formWrapper.addEventListener('change', update_filters_data)
            
      filtersForm.querySelector(".clear-button > *")?.addEventListener("click", () => {
        filtersForm.querySelectorAll(`input`).forEach((i) => i.checked = false)
        FilterManager.filters = {}
      })
      
      window.addEventListener("click", (e) => {
        if(!formWrapper) return
                
        const target = e.target as HTMLElement
        const input = formWrapper.querySelector(`input[name="openPopUpButton"]`)! as HTMLInputElement
        
        if(input.closest(".filters-wrapper")?.contains(target)) return
        
        input.checked = false
      })
    })
</script>

<form class="header" bind:this={formWrapper}>
    <h2>{page_title}</h2>
    
    <span></span>
    
    <div class="wrap">
        <Select  enable_search={false} selected={["Most Reactions"]} maxContent singleOption empty_name="Sort by" icon={2} name="sort" selection_name="sort" 
            data_options={sort_by}/>
        
        <Select  enable_search={false} selected={["Week"]} singleOption maxContent empty_name="Time Period" icon={2} name="timePeriod" selection_name="Period" 
            data_options={time_period_options}/>
        
        <div class="hidden" style="display: none;">
            <Select enable_search={false} maxContent empty_name="Media Type" selected={media_type} icon={2} name="mediaType" selection_name="Media" 
                data_options={[{name: "Image", value: "Image"}, {name: "Video", value: "Video"}]}/>
        </div>

        <div class="alignLeft">
            <div class="filters-wrapper">
                <Button hideMarker type="checkbox" name="openPopUpButton" icon={4} active_route="_" current_route={current_route} no_bg={true} has_dropdown={true}>Filters</Button>
                <div class="formContainer filters-popup" bind:this={filtersForm}>                               
                    <Separator>Base model</Separator>
                    <div class="group">
                        <Select select_list_cards empty_name="Show all" icon={2} name="baseModel" selection_name="Base Models" 
                            data_options={base_models}/>
                    </div>
                    
                    <Separator>Techniques</Separator>
                    <div class="group">
                        <Select select_list_cards empty_name="Show all" icon={2} name="techniques" selection_name="Techniques" 
                            data_options={techniques}/>
                    </div>
                                    
                    <Separator>Modifiers</Separator>
                    <div class="group last">
                        <Button pill name="Show NSFW" value="Show NSFW" type="radioCheckbox" onclick={() => {}}  icon={0}  hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Show NSFW</Button>
                        <Button pill name="Metadata only" value="Metadata only" type="radioCheckbox" onclick={() => {}}  icon={0}  hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Metadata only</Button>
                        <Button pill name="Made On-site" value="Made On-site" type="radioCheckbox" onclick={() => {}}  icon={0} hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Made On-site</Button>
                        <Button pill name="Originals Only" value="Originals Only" type="radioCheckbox" onclick={() => {}}  icon={0} hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Originals Only</Button>
                        <Button pill name="Remixes Only" value="Remixes Only" type="radioCheckbox" onclick={() => {}}  icon={0} hoverColor="#3c3d42" bgHover={true} extraPadding={true}>Remixes Only</Button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</form>

<style>
    h2 {
        padding-bottom: calc(var(--spacing) * 3);
    }
    
    .header {
        --gap: calc(var(--spacing) * 2);
        
        display: block;
        gap: var(--gap);
        padding-bottom: calc(var(--spacing) * 6);
        position: sticky;
        top: 0;
        padding-top: 1rem;
        z-index: 99999;
        background: var(--bgColor);
        
        .wrap {
            display: flex;
            gap: var(--gap);
            align-items: center;
            
            :global(> :nth-child(1)) {
                grid-area: inputs;
            }
            
            :global(> :nth-child(2)) {
                grid-area: inputs_2;
            }
            
            :global(> :nth-child(3)) {
                grid-area: filter;
            }
        }
        
        .alignLeft {
            margin-left: auto;
        }
        
        @media (max-width: 950px) {
            &, .wrap {
                flex-direction: column;
                align-items: baseline;
            }
            
            .wrap {
                display: grid;
                grid-template-columns: 1fr auto;
                grid-template-areas: "inputs filter" "inputs_2 filter";
            }
            
            & > span {
                display: none;
            }
            
            
            .alignLeft {
                margin-left: 0;
            }
        }
    }
    
    .header:has(:global(input[name="openPopUpButton"]:checked)) {
        .filters-popup {
            opacity: 1;
            pointer-events: all;
            scale: 1;
        }
    }
    
    .filters-wrapper {
        position: relative;
        --color: rgba(255,255,255, .6);
        
        .filters-popup {
            background: var(--navBG);
            border: 1px solid rgba(255,255,255, .1);
            
            position: relative;
            padding: 1rem;
            min-width: 400px;
            width: 40vw;
            max-width: 600px;
            min-height: 100px;
            position: absolute;
            top: 100%;
            right: 0;
            z-index: 1;
            border-radius: .4rem;
            opacity: 0;
            pointer-events: none;
            scale: .4;
            transform-origin: top right;
            transition: .12s;
            
            /*max-height: 80vh;
            overflow-y: auto;*/
                        
            .group {
                display: flex;
                flex-wrap: wrap;
                gap: .75rem;
                padding-top: .85rem;
                padding-bottom: 2rem;
                
                &.last {
                    padding-bottom: 0;
                }
            }
            
        }
    }
</style>