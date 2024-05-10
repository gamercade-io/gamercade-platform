<script>
	import NavBar from '$lib/NavBar.svelte';
	import '@picocss/pico/css/pico.indigo.min.css';

	export const prerender = true;
	export const ssr = false;
</script>

<NavBar />

<slot />
