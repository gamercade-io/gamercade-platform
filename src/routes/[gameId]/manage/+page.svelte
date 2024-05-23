<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

Manage Game: {data.gameId}

<!-- TODO: Add these fields which can be adjusted

	Title
	Short Description
	Long Description

	TOOD: Add Submit Changes Button
	TODO: Add Adjust Files Button
	TODO: Add Delete Game Button
-->


<BackButton />
