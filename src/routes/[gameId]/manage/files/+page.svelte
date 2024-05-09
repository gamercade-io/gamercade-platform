<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

Manage Game Files

{data}

<a href="/{data.gameId}/manage" role="button">Back</a>
