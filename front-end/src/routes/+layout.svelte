<script lang="ts">
    import "./layout.css";
    
	import favicon from '$lib/assets/favicon.svg';
		
	import Login from "$lib/components/login.svelte"
	import { userState, getCookie, loginUser, loading_user } from '../lib/state.svelte.ts';
    import { onMount } from 'svelte';
  
    import AppSidebar from "$lib/components/sidebar/app-sidebar.svelte";
    
    import { Separator } from "$lib/components/ui/separator/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    
    import { ModeWatcher } from "mode-watcher";
			        
    onMount(async () => {      
      if (Object.keys(userState).length === 0) {
        let token = getCookie("user_token")
        
        if (!token || token.length < 1) {
          loading_user.loading = false
        }
        
        if (token) {
          await loginUser(token)
        }
      }
      
    })
    
	let { children } = $props();
    
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<ModeWatcher />

{#if Object.keys(userState).length === 0 || loading_user.loading}
    <Login />
    {:else}
    <Sidebar.Provider>
      <AppSidebar />
      <Sidebar.Inset>
        <header class="flex h-16 shrink-0 items-center gap-2">
          <div class="flex items-center gap-2 px-4">
            <Sidebar.Trigger class="-ms-1" />
            <Separator orientation="vertical" class="me-2 data-[orientation=vertical]:h-4" />
          </div>
        </header>
        <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
            {@render children()}
        </div>
      </Sidebar.Inset>
    </Sidebar.Provider>
{/if}


