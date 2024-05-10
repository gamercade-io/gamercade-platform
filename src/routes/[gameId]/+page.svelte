<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

View Game: {data.gameId}

<BackButton />
<a href="/{data.gameId}/manage" role="button">Manage</a>
