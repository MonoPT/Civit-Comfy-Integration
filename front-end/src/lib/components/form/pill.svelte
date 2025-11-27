<script lang="ts">
    let {label, name, type = "radio"} = $props()
</script>

<label class="pill-wrapper">
    <input type="{type}" name="{name}" value="{label}">
    {label}    
</label>

<style>
    .pill-wrapper {
        width: max-content;
        color: var(--color);
        user-select: none;
        font-size: .9em;
        
        border-radius: 1000px;
        padding-inline: .8rem;
        padding-block: .1rem;
        background: rgba(255,255,255, .05);
        cursor: pointer;
        transition: .1s;
        
        &:has(:checked) {
            background: #245292;
            color: #fff;
        }
        
        input {
            appearance: none;
            color: inherit;
        }
    }
</style>