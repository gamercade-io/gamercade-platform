<script>
	import '@picocss/pico/css/pico.indigo.min.css';
	import NavBar from '../NavBar.svelte';

	export const prerender = true;
	export const ssr = false;
</script>

<NavBar />

<slot />
