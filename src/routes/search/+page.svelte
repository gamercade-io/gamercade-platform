<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>Search</h1>

<main class="container">
	<fieldset>
		<input name="gameTitle" placeholder="Search by Game Title..." type="search" />
	</fieldset>

	<BackButton />
</main>
