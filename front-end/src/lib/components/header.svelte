<script lang="ts">
    import Button from "./button.svelte"
    let {current_route = ""} = $props()
    import Separator from "$lib/components/form/separator.svelte"
    import Pill from "$lib/components/form/pill.svelte"
    import FilterManager from "$lib/api/filterManager"
    import Select from "$lib/components/form/select.svelte";

    import { onMount } from "svelte";
    
    let openPopUp = $state(false)
    let filtersForm: HTMLFormElement;
        
    onMount(() => {
      FilterManager.filters = {} // Clear filters from other pages
      
      filtersForm.addEventListener('input', (event) => {
          const input = event.target as HTMLInputElement;
          
          let inputs = filtersForm.querySelectorAll(`[name="${input.name}"]`)
          
          let values: string[] = []
          
          inputs.forEach((e) => {
            let input = e as HTMLInputElement
            
            if (!input.checked) return
            
            values.push(input.value)
          })
          
          FilterManager.update_filter(input.name, values)          
      });
      
      filtersForm.querySelector(".clear-button > *")?.addEventListener("click", () => {
        filtersForm.querySelectorAll(`input`).forEach((i) => i.checked = false)
        FilterManager.filters = {}
      })
    })
</script>

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
        <div class="filters-wrapper">
            <Button onclick={() => openPopUp = !openPopUp} icon={4} active_route="_" current_route={current_route} no_bg={true} has_dropdown={true} dropdown_is_open={openPopUp}>Filters</Button>
            <form class="filters-popup" class:openPopUp bind:this={filtersForm}>
                <Separator>Sort by</Separator>
                <div class="group">
                    <Pill label="Most Reactions" name="sort" />
                    <Pill label="Most Comments" name="sort" />
                    <Pill label="Newest" name="sort" />
                </div>
                
                <Separator>Time period</Separator>
                <div class="group">
                    <Pill label="Day" name="period" />
                    <Pill label="Week" name="period" />
                    <Pill label="Month" name="period" />
                    <Pill label="Year" name="period" />
                    <Pill label="All Time" name="period" />
                </div>
                
                <Separator>Base model</Separator>
                <div class="group">
                    <Pill label="AuraFlow" name="baseModel" type="checkbox" />
                    <Pill label="Chroma" name="baseModel" type="checkbox" />
                    <Pill label="CogVideoX" name="baseModel" type="checkbox" />
                    <Pill label="Flux.1 S" name="baseModel" type="checkbox" />
                    <Pill label="Flux.1 D" name="baseModel" type="checkbox" />
                    <Pill label="Flux.1 Krea" name="baseModel" type="checkbox" />
                    <Pill label="Flux.1 Kontext" name="baseModel" type="checkbox" />
                    <Pill label="Flux.2 D" name="baseModel" type="checkbox" />
                    <Pill label="Hi Dream" name="baseModel" type="checkbox" />
                    <Pill label="LTXV" name="baseModel" type="checkbox" />
                </div>
                
                <Separator>Media type</Separator>
                <div class="group">
                    <Pill label="Image" name="mediaType" type="checkbox" />
                    <Pill label="Video" name="mediaType" type="checkbox" />
                </div>
                
                <Separator>Modifiers</Separator>
                <div class="group">
                    <Pill label="Metadata only" name="Modifiers" type="checkbox" />
                    <Pill label="Made On-site" name="Modifiers" type="checkbox" />
                    <Pill label="Originals Only" name="Modifiers" type="checkbox" />
                    <Pill label="Remixes Only" name="Modifiers" type="checkbox" />
                </div>
                
                <div class="clear-button">
                    <Button fullWidth icon={0} >Clear all filters</Button>
                </div>
            </form>
        </div>
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
            
            max-height: 80vh;
            overflow-y: auto;
            
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