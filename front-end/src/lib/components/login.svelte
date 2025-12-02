<script lang="ts">
    import { loginUser, loading_user } from '../state.svelte.ts';
    import Input from './form/input.svelte';
    import Spinner from './spinner.svelte';
        
    async function loginUserBtn(value: string) {
      loginUser(value)
    }
    
</script>

<div id="login">
    {#if loading_user.loading} 
        <div class="Images-Loading-Wrapper">
            <Spinner size={35} tickness={3} />
            <p>Loading user data</p>
        </div>
    {:else}
        <div class="wrap">
            <h2>You're Logged out</h2>
            <h3>To login, follow the instruction bellow:</h3>
            <ol>
                <li>Go to: <a href="https://civitai.com">https://civitai.com</a></li>
                <li>Login into your account</li>
                <li>Open the devtools (Ctrl + Shift + I)</li>
                <li>Navigate to cookies section</li>
                <li>Copy value of the cookie "__Secure-civitai-token" and paste it bellow</li>
            </ol>
            
            <Input btn_onclick={() => loginUserBtn}/>
            
            {#if loading_user.error}
                <h4 class="errorMsg">The provided token is not valid!</h4>
            {/if}
        </div>
    {/if}
</div>

<style>
    h2, h3, ol, h4 {
        text-align: center;
        width: max-content;
    }
    
    ol {
        padding-top: .6rem;
        padding-left: 1.5rem;
        margin-bottom: 2rem;
    }
    
    ol, h3 {
        opacity: .75;
    }
    
    li {
        text-align: left;
        line-height: 1.7;
        font-weight: 300;
        font-size: .9rem;
    }
    
    #login {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        min-height: 100vh;
    }
    
    h2 {
        font-weight: 600;
        padding-bottom: 1.4rem;
        font-size: 2.2rem;
    }
    
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
    
    h4 {
        margin-inline: auto;
        margin-top: 1.4rem;
        font-size: .9em;
        color: #eb4034;
        opacity: .9;
    }
</style>