<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

Manage Game Files

{data}

<BackButton />
