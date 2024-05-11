<script lang="ts">
	import GoToButton from '$lib/GoToButton.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	function refresh() {
		// TODO: This
	}
</script>

<h1>Top</h1>

<main class="container">
	<button on:click={refresh}>Refresh</button>
	<GoToButton target="/search" text="Search" />
</main>
