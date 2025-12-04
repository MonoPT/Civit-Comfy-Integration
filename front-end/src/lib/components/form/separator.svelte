<div class="separator-wrapper">
    <h2><slot></slot></h2>
    <div class="line"></div>
</div>

<style>
    .separator-wrapper {
        display: grid;
        grid-template-columns: auto 1fr;
        align-items: center;
        gap: .8rem;
    }
    
    .line {
        display: block;
        width: 100%;
        height: 1px;
        background: rgba(255,255,255, .7);
        border-radius: 10000px;
        opacity: .3;
    }
    
    h2 {
        font-weight: 350;
        width: max-content;
        color: rgba(255,255,255, .7);
        font-size: 1em;
    }
</style>