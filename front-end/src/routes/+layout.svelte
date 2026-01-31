<script lang="ts">
    import "./layout.css";
    
	import favicon from '$lib/assets/favicon.svg';
		
	import Login from "$lib/components/login.svelte"
	import { userState, getCookie, loginUser, loading_user } from '../lib/state.svelte.ts';
    import { onMount } from 'svelte';
  
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
    {@render children()}
{/if}


