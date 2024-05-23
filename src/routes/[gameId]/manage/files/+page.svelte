<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

Manage Game Files

<!-- TODO: Add these:
	Upload GameRom Button
	Upload GameImage Button
	Delete GameRom Button
	Delete GameImage Button
-->

{data}

<BackButton />
