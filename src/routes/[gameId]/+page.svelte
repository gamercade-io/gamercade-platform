<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from './$types';
	import GoToButton from '$lib/GoToButton.svelte';

	export let data: PageData;
</script>

View Game: {data.gameId}

<BackButton />

<GoToButton target="/{data.gameId}/manage" text="Manage" />
