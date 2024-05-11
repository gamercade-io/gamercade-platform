<script>
	import NavBar from '$lib/NavBar.svelte';
	import '@picocss/pico/css/pico.indigo.min.css';
	import { getGlobalPermissionLevels, getGlobalTags } from '$lib';
	import { onMount } from 'svelte';

	export const prerender = true;
	export const ssr = false;

	onMount(() => {
		getGlobalTags();
		getGlobalPermissionLevels();
	});
</script>

<NavBar />

<slot />
