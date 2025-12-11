<script lang="ts">
    import { onMount } from "svelte";
    import api from "$lib/api"
    import {user_token} from "$lib/state.svelte"
    import {ImageGallery, type Image} from "$lib/api/imageGallery";
    import Spinner from "$lib/components/spinner.svelte";
    import Controls from "$lib/components/visualizerContros.svelte"
    
    type GenData = {
      tags: {automated:boolean,concrete:boolean,downVotes:number,id:number,lastUpvote:string, name:string, needsReview: boolean, nsfwLevel:number, score: number, type:string, upVotes: number}[],
      image: string,
      media_type: "image" | "video",
      media_data: null | Image,
      gen_data: any | null,
      landscape: boolean
    }
    
    let isLoadingData = $state(false)
    let isOpen = $state(false)
    let data: GenData = $state({
      tags: [],
      image: "",
      media_type: "image",
      media_data: null,
      gen_data: null,
      landscape: false
    })
        
    let checkedOverflow = false
    
    onMount(() => 
      window.addEventListener("loadMediaVisualizer", async (e) => {
        if (isLoadingData) return
        
        data.tags = []
        data.image = ""
        data.gen_data = null
        data.landscape = false
                
        checkedOverflow = false
        isLoadingData = true
        //@ts-ignore
        let {id, creator_id, image_uuid} = e.detail
        isOpen = true;
                
        const media_data = ImageGallery.images.find((img) => img.uuid === image_uuid)!
        const mediaType = media_data.media_type as "image" | "video"
        
        const uuid = media_data.uuid
        const src_proxy = `${api.endpoint}/media_proxy?id=${uuid}`
        
        let src = `${src_proxy}&media_type=${mediaType}`; 
                
        data.media_type = mediaType
        data.image = src
        data.media_data = media_data
                
        if (media_data.width > media_data.height) {
          data.landscape = true
        }
                
        const req = await fetch(api.media_data(user_token.token, id, creator_id));
        isLoadingData = false
        
        if (req.status !== 200) {
          console.log(await req.text())
          alert("Error loading image data")
          return
        }
        
        const resp_data = await req.json()
                
        data.tags = resp_data.votable_tags
        data.gen_data = resp_data.generation_data
      })
    )
    
    onMount(() => {
      const container = document.querySelector("#mediaVisualizer .metaWrapper")!
      
      const observer = new MutationObserver((mutationsList, observer) => {
          for (const mutation of mutationsList) {
              if (mutation.type === 'childList') {
                  if (checkedOverflow) return
                  
                  (mutation.target as HTMLElement).querySelectorAll(".wrapperContainer .prompt, .wrapperContainer .tagWrapper").forEach((element) => {
                    checkedOverflow = true;
                    let has_overflow = element.scrollHeight > element.clientHeight || element.scrollWidth > element.clientWidth;
                    
                    if (!has_overflow) {
                      element.setAttribute("data-overflow-false", "");
                    }
                  })
              }
          }
      });

      const config = {
          childList: true,      // observar adição ou remoção de filhos
          attributes: false,     // observar alterações de atributos
          subtree: true         // observar também todos os descendentes
      };
      
      observer.observe(container, config);
    })
    
    let meta_is_open = false
    
    onMount(() => {
      window.addEventListener("visualizerToggleMeta", () => {
        meta_is_open = !meta_is_open
        
        document.getElementById("mediaVisualizer")?.removeAttribute("data-open-metadata")
        
        if (meta_is_open) {
          document.getElementById("mediaVisualizer")?.setAttribute("data-open-metadata", "")
        }
      })
      
      window.addEventListener("downloadVisualizerMedia", async () => {
        const media = document.querySelector("#mediaVisualizer :is(img, video source)")!
        
        //@ts-ignore
        const response = await fetch(media.src);
        const blob = await response.blob();
    
        const blobUrl = URL.createObjectURL(blob);
        
        const a = document.createElement("a");
        a.href = blobUrl;
        a.download = `${data.media_data?.id}.${data.media_type === "image" ? 'jpg' : 'mp4'}`;
        a.click();
      })
      
      window.addEventListener("closeVisualizer", () => {
        isOpen = false
        meta_is_open = false
        data.media_type = "image"
        document.getElementById("mediaVisualizer")?.removeAttribute("data-open-metadata")
      })
    })
</script>

<div class="container" class:open={isOpen} id="mediaVisualizer">
    <div class="imageWrapper">
        {#if data.media_type === "image"}
            <img src={data.image} alt="" class:landscape={data.landscape}>
            {:else}
            <video autoplay muted loop class:landscape={data.landscape}>
                <track kind="captions" />
                <source src={data.image}/>
            </video>
        {/if}
        
            
        <div class="reactionsContainer">
            {#if data.media_data && data.media_data.stats}
                <span>👍 {data.media_data.stats.likeCountAllTime}</span>
                <span>❤️ {data.media_data.stats.heartCountAllTime}</span>
                <span>😂 {data.media_data.stats.laughCountAllTime}</span>
                <span>😢 {data.media_data.stats.cryCountAllTime}</span>
            {/if}
        </div>
        
        <div class="card mobileFastActions">
            <Controls is_mobile={true} />
        </div>
    </div>
    
    <div class="metaWrapper">
        {#if isLoadingData}
            <div class="Images-Loading-Wrapper">
                <Spinner size={35} tickness={3} />
                <p>Loading data</p>
            </div>
        {:else}
            
            <div class="card mobileFastActions">
                <Controls open={true} is_mobile={true} />
            </div>
            
            <div class="card cardDesktopFastActions">
                <Controls />
            </div>
        
            {#if data.tags.length > 0}
                <div class="card tagContainer">
                    <h2>Tags</h2>
                    
                    <div class="wrapperContainer tagWrapper">
                        {#each data.tags as tag}
                            <span data-squared-pill class="weigth2" data-tag-id={tag.id}>{tag.name}</span>
                        {/each}
                    </div>
                    <label data-showMore>Show <input type="checkbox" hidden></label>
                </div>
            {/if}
            
            
            {#if data.gen_data?.meta}
                <div class="card">
                    <h2>
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-forms "><path d="M12 3a3 3 0 0 0 -3 3v12a3 3 0 0 0 3 3"></path><path d="M6 3a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3"></path><path d="M13 7h7a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-7"></path><path d="M5 7h-1a1 1 0 0 0 -1 1v8a1 1 0 0 0 1 1h1"></path><path d="M17 12h.01"></path><path d="M13 12h.01"></path></svg>
                        Generation data
                    </h2>
                    
                    {#if data.gen_data?.resources.length > 0}
                        <div class="wrapperContainer">
                            <h3>Resources used</h3>
                            
                            <div class="resourceWrapper">
                                {#each data.gen_data?.resources as resource}
                                    <div class="resource">
                                        <div class="left">
                                            <span class="res_name">{resource.modelName}</span>
                                            <span class="res_version">{resource.versionName}</span>
                                        </div>
                                        <div class="rigth">
                                            <span data-squared-pill>{resource.modelType}</span>
                                            {#if resource.strength}
                                                <span data-squared-pill class="weigth">{resource.strength}</span>
                                            {/if}
                                        </div>
                                    </div>
                                {/each}
                                <label data-showMore>Show <input type="checkbox" hidden></label>
                            </div>
                        </div>
                    {/if}
                    
                    <div class="wrapperContainer">
                        {#if data.gen_data?.meta.prompt}
                            <h3>Prompt</h3>
                            <p class="prompt">{data.gen_data?.meta.prompt}</p>
                            <label data-showMore>Show <input type="checkbox" hidden></label>
                        {/if}
                    </div>
                    
                    <div class="wrapperContainer">
                        {#if data.gen_data?.meta.negativePrompt}
                            <h3>Negative Prompt</h3>
                            <p class="prompt">{data.gen_data?.meta.negativePrompt}</p>
                            <label data-showMore>Show <input type="checkbox" hidden></label>
                        {/if}
                    </div>
                    
                    {#if data.gen_data?.meta}
                        <div class="wrapperContainer otherMetadata">
                            <h3>Other metadata</h3>
                            
                            <div class="othermetadatawrapper">
                                {#if data.gen_data?.meta.cfgScale}
                                    <span data-squared-pill data-uppercase>cfgScale: {data.gen_data?.meta.cfgScale}</span>
                                {/if}
                                {#if data.gen_data?.meta.steps}
                                    <span data-squared-pill data-uppercase>steps: {data.gen_data?.meta.steps}</span>
                                {/if}
                                {#if data.gen_data?.meta.sampler}
                                    <span data-squared-pill data-uppercase>sampler: {data.gen_data?.meta.sampler}</span>
                                {/if}
                                {#if data.gen_data?.meta.seed}
                                    <span data-squared-pill data-uppercase>seed: {data.gen_data?.meta.seed}</span>
                                {/if}
                                {#if data.gen_data?.meta.Size}
                                    <span data-squared-pill data-uppercase>size: {data.gen_data?.meta.Size}</span>
                                {/if}
                            </div>
                        </div>
                    {/if}
                </div>
            {/if}
            
            {#if (data.gen_data?.tools?.length || 0 + data.gen_data?.techniques?.length || 0) > 0}
                <div class="card">
                    <h2>
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-chart-bubble "><path d="M6 16m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path><path d="M16 19m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path><path d="M14.5 7.5m-4.5 0a4.5 4.5 0 1 0 9 0a4.5 4.5 0 1 0 -9 0"></path></svg>
                        Process
                    </h2>
                    {#if data.gen_data?.tools.length > 0}
                        <h3>Tools</h3>
                        <div class="cardWrapper">
                            {#each data.gen_data?.tools as tool}
                                <span>{tool.name}</span>
                            {/each}
                        </div>
                    {/if}
                    
                    {#if data.gen_data?.techniques.length > 0}
                        <h3>Techniques</h3>
                        <div class="cardWrapper transparentTag">
                            {#each data.gen_data?.techniques as technique}
                                <span>{technique.name}</span>
                            {/each}
                        </div>
                    {/if}
                </div>
            {/if}
        {/if}
    </div>
</div>


<style>     
    .Images-Loading-Wrapper {
        display: flex;
        flex-direction: column;
        margin-top: 2rem;
        justify-content: center;
        align-items: center;
        
        p {
            margin-top: .7rem;
        }
    }
    
    .otherMetadata:not(:has(.othermetadatawrapper span:nth-child(1))) {
        display: none;
    }
    
    .tagContainer {
        --l: 2;
        &:has([data-showMore] input:checked) {
            --l: 10000000000;
            
            .wrapperContainer:after {
                display: none;
            }
        }
        
        .wrapperContainer {
            
            display: flex;
            flex-wrap: wrap;
            gap: .2rem;
            position: relative;
            
            span {
                font-weight: 300;
                font-size: .85em;
                
            }
            
            overflow: hidden;
            max-height: calc((.85em + .2em * 2) * var(--l)  + .4em * var(--l) );
            
            &:after {
                content: '';
                position: absolute;
                bottom: 0;
                background: linear-gradient(180deg, transparent 0%,  color-mix(in srgb, var(--navBG) 90%, transparent 10%) 100%);
                min-height: 90%;
                left: 0;
                width: 100%;
                pointer-events: none;
                z-index: 0;
            }
            
            :global(&:has([data-overflow-false])) {
                [data-showMore], &:after {
                    display: none;
                }
            }
        }
    }
    
    .othermetadatawrapper {
        display: flex;
        flex-wrap: wrap;
        gap: .5rem;
        font-size: .75em;
    }
    
    [data-squared-pill] {
        background: #1971C2;
        padding-inline: .5em;
        padding-block: .2em;
        border-radius: .2em;
        color: rgba(255,255,255,.75);
        
        &.weigth {
            background: rgba(255,255,255,.1);
        }
        
        &[data-uppercase] {
            text-transform: uppercase;
        }
    }
    
    .resourceWrapper {
        display: flex;
        flex-direction: column;
        row-gap: .8rem;
        
        .resource {
            --max-lines: 1;
            --max-h: calc(.85em * 1.5 * var(--max-lines));
            
            display: grid;
            grid-template-columns: 1fr auto;
            align-items: baseline;
            gap: .5rem;
            
            span {
                font-size: .85em;
                
                &.res_version {
                    font-size: .7em;
                }
            }
            
            span.res_name {
                text-decoration: underline;
            }
            
            .res_name {
                overflow: hidden;
                display: -webkit-box;
                -webkit-line-clamp: var(--max-lines);
                line-clamp: var(--max-lines);
                -webkit-box-orient: vertical;
            }
            
            .left {
                display: flex;
                flex-direction: column;
                gap: .2rem;
                
                span.res_version {
                    color: rgba(255,255,255, .5);
                }
            }
            
            .rigth {
                display: flex;
                align-items: center;
                gap: .35rem;
            }
        }
        
        &:has(.resource:nth-child(4)){
            &:not(:has([data-showMore] input:checked))
            
            .resource:nth-child(n + 4) {
                display: none;
            }
        }
        
        &:not(:has(.resource:nth-child(n + 4))) {
            [data-showMore] {
                display: none;
            }
        }
    }
    
    
    .wrapperContainer {
        --max-lines: 4;
        --max-h: calc(.85em * 1.5 * var(--max-lines)); 
        line-height: 1.3;               
    }
    
    [data-showMore] {
        font-size: .85em;
        cursor: pointer;
        color: color-mix(in srgb, #1971C2 80%, #fff 20%);
        font-weight: 500;
        
        &:after {
            content: 'more'
        }
        
        &:has(input:checked) {
            &:after {
                content: 'less'
            }
        }
    }
    
    .prompt {
        position: relative;
        font-size: .85em;
        user-select: text;
        opacity: .65;
        font-weight: 300;
        line-height: 1.3;
        min-height: max-content;
        
        &:has(+ [data-showMore] input:not(:checked)) {
            display: -webkit-box;
            -webkit-line-clamp: var(--max-lines);
            line-clamp: var(--max-lines);
            -webkit-box-orient: vertical;
            
            overflow: hidden;
            max-height: var(--max-h);
            
            &:after {
                content: '';
                position: absolute;
                bottom: 0;
                background: linear-gradient(180deg, transparent 0%,  color-mix(in srgb, var(--navBG) 90%, transparent 10%) 100%);
                min-height: 80%;
                left: 0;
                width: 100%;
                pointer-events: none;
            }
        }
                
        :global(&[data-overflow-false]) {
            &:after {
                display: none;
            }
            
            + [data-showMore] {
                display: none;
            }
        }
    }
    
    .card {
        background: var(--navBG);
        border: 1px solid rgba(255,255,255, .1);
        
        position: relative;
        padding: 1rem;
        width: 100%;
        min-height: 100px;
        border-radius: .4rem;
        min-height: max-content;
        width: 100%;
        
        max-width: 80vw;
                
        h2 {
            padding-top: 0;
            padding-bottom: calc(var(--spacing) * 4);
            display: flex;
            align-items: center;
            gap: .4rem;
            
            svg {
                width: 2rem;
                aspect-ratio: 1 / 1;
                display: inline-block;
            }
        }
        
        h3 {
            padding-top: calc(var(--spacing) * 6);
            padding-bottom: calc(var(--spacing) * 2);
        }
        
        h2 + h3, h2 + .wrapperContainer h3 {
            padding-top: 0;
        }
    }
    
    .cardWrapper {
        display: flex;
        flex-wrap: wrap;
        gap: calc(var(--spacing) * 2);
        
        span {
            font-size: .8em;
            background: rgba(255,255,255,.15);
            border-radius: calc(var(--spacing) * .7);
            padding: calc(var(--spacing) * 1.5);
            padding-inline: calc(var(--spacing) * 4);
        }
    }
    
    .cardWrapper {
        span {
            background: #1971C2;
            border-radius: 1000px;
            border: 1px solid color-mix(in srgb, #1971C2 60%, #fff 40%);
            box-sizing: content-box;
            color: rgba(255,255,255,.85);
            font-size: .9em;
        }
        
        &.transparentTag {
            gap: 1rem;
            span {
                padding: 0;
                border: none;
                background: transparent;
            }
        }
    }
    
    .metaWrapper {
        height: 100%;
        overflow-y: auto;
        padding-top: calc(var(--spacing) * 6 + 2rem);
        display: flex;
        flex-direction: column;
        gap: calc(var(--spacing) * 5);
        padding-bottom: 2rem;
        
        &:has(.Images-Loading-Wrapper) {
            justify-content: center;
        }
    }
    
    
    
    .imageWrapper {
        --spaceTop: 0px;
        
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100vh;
        flex-direction: column;
        gap: 1rem;
        padding-block: calc(var(--spacing) * 3);
        
        
        
        img, video {
            display: block;
            height: calc(100% - var(--spacing) * 8 - 2rem * 2 - var(--spaceTop));
            width: auto;
            
            &.landscape {
                height: auto;
                width: 100%;
            }
        }
    }
    
    :global(body:has(#mediaVisualizer.open)) {
        overflow: hidden;
    }
    
    .reactionsContainer {
        position: absolute;
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 2;
        bottom: 0rem;
        padding: 1rem;
        gap: calc(var(--spacing) * 1);
        width: 100%;
        flex-wrap: wrap;
        pointer-events: none;
        
        span {
            display: flex;
            align-items: center;
            gap: calc(var(--spacing) * 2);
            background: rgba(0,0,0,.6);
            padding: calc(var(--spacing) * 1);
            padding-inline: calc(var(--spacing) * 2);
            border-radius: calc(var(--spacing) * 1);
            width: max-content;
            font-size: .8em;
            pointer-events: all;
        }   
    }
    
    .mobileFastActions {
        display: none;
        max-width: min(500px, 80vw);
    }
    
    .container {
        .imageWrapper, .metaWrapper {
            width: calc(70vw / 2);
            
            @media (max-width: 1100px) {
                width: calc(100vw / 2);
            }
        }
        
        @media (max-width: 850px) {
            .imageWrapper, .metaWrapper {
                width: 100vw;
                transition: .2s;
            }
            
            .mobileFastActions {
                display: block;
            }
            
            .cardDesktopFastActions {
                display: none;
            }
            
            .metaWrapper {
                position: absolute;
                z-index: 99;
                align-items: center;
            }
            
            .imageWrapper {
                padding: 0;
                --spaceTop: 100px;
                
                img, video {
                    
                }
                
                
                .reactionsContainer  {
                    position: relative;
                    padding: 0;
                }
            }
            
            &:not([data-open-metadata]) .metaWrapper {
                transform: translateY(-100vh);
            }
            
            :global(&[data-open-metadata]) .imageWrapper {
                transform: translateY(-100vh);
            }
        }
    }
    
    .container {
        
        
        display: flex;
        
        gap: calc(var(--spacing) * 16);
        padding-inline: calc(var(--spacing) * 4);
        align-items: center;
        justify-content: center;
        position: fixed;
        top: 0;
        left: 0;
        background: rgba(0,0,0,.65);
        
        z-index: 999999999999999999;
        width: 100vw;
        height: 100vh;
        transform-origin: center;
        transition: .12s;
        backdrop-filter: blur(12px);
        
        &:not(.open) {
            pointer-events: none;
            opacity: 0;
            scale: .4;
        }
        
    }
</style>