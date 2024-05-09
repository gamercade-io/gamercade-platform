<script lang="ts">
	import type { PageData } from '../create/$types';

	export let data: PageData;
</script>

<main style="container">
	<h1>Create New Game</h1>
	<label for="Game Title">Game Title</label>
	<input name="gameTitle" placeholder="Game Title" />

	<label for="Short Description">Short Description</label>
	<input name="shortDescription" placeholder="Short Description" />

	<label for="Long Description">Long Description</label>
	<input name="longDescription" placeholder="Long Description" />

	<a href="/dashboard" role="button">Back</a>
	<button>Create</button>
</main>
