<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>Search</h1>

<main class="container">
	<fieldset>
		<input name="gameTitle" placeholder="Search by Game Title..." type="search" />
	</fieldset>

	<a href="/arcade/top" role="button">Back</a>
</main>
