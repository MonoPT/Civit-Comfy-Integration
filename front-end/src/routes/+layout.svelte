<script lang="ts">
    import "./layout.css";
    
    import { Toaster } from "$lib/components/ui/sonner/index.js";
    
	import favicon from '$lib/assets/favicon.svg';
		
	import ModelDownloadDialog from "$lib/composables/model_download_dialogue/dialogue.svelte"
	
	import AssetPreview from "$lib/composables/asset_preview/preview.svelte"
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
    import * as Select from "$lib/components/ui/select/index.js";
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<Toaster />
<ModeWatcher />
<AssetPreview />
<ModelDownloadDialog />

{#if Object.keys(userState).length === 0 || loading_user.loading}
    <Login />
    {:else}
    <Sidebar.Provider>
      <AppSidebar />
      <Sidebar.Inset>
        <header class="flex h-16 shrink-0 items-center gap-2 sticky top-0 z-50" style="background: var(--background);">
          <div class="flex items-center gap-2 px-4 w-full">
            <Sidebar.Trigger class="-ms-1" />
            <Separator orientation="vertical" class="me-2 data-[orientation=vertical]:h-4" />
            <div class="header-content-nav w-full">
                
            </div>
          </div>
        </header>
        <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
            {@render children()}
        </div>
      </Sidebar.Inset>
    </Sidebar.Provider>
{/if}


