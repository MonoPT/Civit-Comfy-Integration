<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import Header from '$lib/components/header.svelte';
	import { page } from '$app/stores';

    let currentRoute = $state( $page.url.pathname);
    console.log("Route: " + currentRoute)
	
	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<Header current_route={currentRoute} />
{@render children()}
