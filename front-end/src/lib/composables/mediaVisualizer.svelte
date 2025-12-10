<script>
    let isOpen = $state(true)
</script>

<div class="container" class:open={isOpen} id="mediaVisualizer">
    <div class="imageWrapper">
        <img src="https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/657f5ee5-b502-4af9-bedc-cbd5e469af7c/original=true,quality=90/113251212.jpeg" alt="">
        <div class="reactionsContainer">
            <span>👍 1</span>
            <span>❤️ 1</span>
            <span>😂 1</span>
            <span>😢 1</span>
        </div>
    </div>
    <div class="metaWrapper">
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Aliquam earum excepturi voluptatibus porro rem modi in, vitae nobis, numquam, voluptas velit laboriosam esse odit enim illo quia et nesciunt laudantium quo! Fugiat sit delectus placeat dicta laboriosam dolore consectetur perspiciatis saepe unde, ipsum vitae eum harum, voluptate sequi similique. Neque?
        <div class="tags">
            {#each [1,2,3,4,5,6,7,8,9,1,1,1,1,1,1,1,1,1] as i}
                <span>tag {i}</span>
            {/each}
        </div>
        
        <div class="card">
            <h2>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-forms "><path d="M12 3a3 3 0 0 0 -3 3v12a3 3 0 0 0 3 3"></path><path d="M6 3a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3"></path><path d="M13 7h7a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-7"></path><path d="M5 7h-1a1 1 0 0 0 -1 1v8a1 1 0 0 0 1 1h1"></path><path d="M17 12h.01"></path><path d="M13 12h.01"></path></svg>
                Generation data
            </h2>
        </div>
        
        <div class="card">
            <h2>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-chart-bubble "><path d="M6 16m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path><path d="M16 19m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0"></path><path d="M14.5 7.5m-4.5 0a4.5 4.5 0 1 0 9 0a4.5 4.5 0 1 0 -9 0"></path></svg>
                Process
            </h2>
            <h3>Tools</h3>
            <div class="cardWrapper">
                <span>GPT Image 1</span>
            </div>
            <h3>Techniques</h3>
            <div class="cardWrapper transparentTag">
                <span>Text2IMG</span>
            </div>
        </div>
    </div>
</div>


<style> 
    .card {
        background: var(--navBG);
        border: 1px solid rgba(255,255,255, .1);
        
        position: relative;
        padding: 1rem;
        width: 100%;
        min-height: 100px;
        border-radius: .4rem;
                
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
    }
    
    .tags, .cardWrapper {
        display: flex;
        flex-wrap: wrap;
        gap: calc(var(--spacing) * 2);
        
        span {
            font-size: .9em;
            background: rgba(255,255,255,.15);
            border-radius: calc(var(--spacing) * .7);
            padding: calc(var(--spacing) * 1.5);
            padding-inline: calc(var(--spacing) * 4);
        }
    }
    
    .cardWrapper {
        padding-top: calc(var(--spacing) * 2);
        padding-bottom: calc(var(--spacing) * 4);
        
        span {
            background: #1971C2;
            border-radius: 1000px;
            border: 1px solid color-mix(in srgb, #1971C2 60%, #fff 40%);
            box-sizing: content-box;
            color: rgba(255,255,255,.85);
            font-size: .9em;
        }
        
        &.transparentTag {
            gap: 2rem;
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
    }
    
    .imageWrapper, .metaWrapper {
        max-width: calc(70vw / 2);
    }
    
    .imageWrapper {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100vh;
        flex-direction: column;
        gap: 1rem;
        padding-block: calc(var(--spacing) * 3);
        
        
        
        img {
            display: block;
            height: calc(100% - var(--spacing) * 8 - 2rem * 2);
            width: auto;
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