<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from './$types';
	import GoToButton from '$lib/GoToButton.svelte';

	export let data: PageData;
</script>

View Game: {data.gameId}

<!-- TODO: Display these fields:
Title
Short Description
Long Description
Game Size
Total Downloads
Average Rating

	TODO: Add Download Button
		- Download replaced with Play if game exists
 -->

<BackButton />

<GoToButton target="/{data.gameId}/manage" text="Manage" />
