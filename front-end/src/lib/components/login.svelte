<script lang="ts">
    import { loginUser, loading_user } from '../state.svelte.ts';
    
    // Icons
    import {SendHorizontal, SunIcon, MoonIcon} from "@lucide/svelte"
    
    // Components
    import { Button } from "$lib/components/ui/button/index.js";
    import * as ButtonGroup from "$lib/components/ui/button-group/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { toggleMode } from "mode-watcher";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    
    async function loginUserBtn(e: Event) {
      const value = document.querySelector<HTMLInputElement>("input[name='loginCookie']")!.value
      loginUser(value)
    }

</script>

<div id="login">
    {#if loading_user.loading} 
        <div class="Images-Loading-Wrapper">
            <Spinner class="size-8" />
            <p>Loading user data</p>
        </div>
    {:else}
        <form class="wrap" onsubmit={loginUserBtn}>
            <h2>You're Logged out</h2>
            <h3>To login, follow the instruction bellow:</h3>
            <ol>
                <li>Go to: <a href="https://civitai.com" target="_blank">https://civitai.com</a></li>
                <li>Login into your account</li>
                <li>Open the devtools (Ctrl + Shift + I)</li>
                <li>Navigate to cookies section</li>
                <li>Copy value of the cookie "__Secure-civitai-token" and paste it bellow</li>
            </ol>
                                    
            <ButtonGroup.Root class="w-full">
              <Input required placeholder="Paste Cookie here" name="loginCookie" />
              <Button variant="outline" type="submit" size="icon" aria-label="Search">
                <SendHorizontal />
              </Button>
            </ButtonGroup.Root>
            
            {#if loading_user.error}
                <h4 class="errorMsg">The provided token is not valid!</h4>
            {/if}
        </form>
        
        <Button onclick={toggleMode} variant="outline" size="icon" class="absolute bottom-2 left-2">
          <SunIcon
            class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 !transition-all dark:scale-0 dark:-rotate-90"
          />
          <MoonIcon
            class="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 !transition-all dark:scale-100 dark:rotate-0"
          />
          <span class="sr-only">Toggle theme</span>
        </Button>
        
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