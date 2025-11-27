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
        background: var(--color);
        border-radius: 10000px;
        opacity: .3;
    }
    
    h2 {
        font-weight: 500;
        width: max-content;
        color: var(--color);
    }
</style>