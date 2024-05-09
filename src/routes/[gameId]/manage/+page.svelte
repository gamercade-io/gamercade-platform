<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

Manage Game: {data.gameId}

<a href="/{data.gameId}" role="button">Back</a>
