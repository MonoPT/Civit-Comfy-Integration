<div class="container open" id="mediaVisualizer">
    <div class="imageWrapper">
        <img src="https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/657f5ee5-b502-4af9-bedc-cbd5e469af7c/original=true,quality=90/113251212.jpeg" alt="">
        <div class="reactionsWrapper">
            <span>1</span>
            <span>1</span>
            <span>1</span>
            <span>1</span>
            <span>1</span>
        </div>
    </div>
    <div class="metaWrapper">
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Aliquam earum excepturi voluptatibus porro rem modi in, vitae nobis, numquam, voluptas velit laboriosam esse odit enim illo quia et nesciunt laudantium quo! Fugiat sit delectus placeat dicta laboriosam dolore consectetur perspiciatis saepe unde, ipsum vitae eum harum, voluptate sequi similique. Neque?
    </div>
</div>


<style> 
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
            height: calc(100% - var(--spacing) * 8);
            width: auto;
        }
    }
    
    :global(body:has(#mediaVisualizer.open)) {
        overflow: hidden;
    }
    
    .container {
        display: flex;
        
        gap: calc(var(--spacing) * 4);
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