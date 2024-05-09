<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

View Game: {data.gameId}

<a href="/" role="button">Back</a>
<a href="/{data.gameId}/manage" role="button">Manage</a>
