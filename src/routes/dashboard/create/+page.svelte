<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from '../create/$types';
	import { invoke } from '@tauri-apps/api/tauri';

	export let data: PageData;

	// Fields
	let gameTitle: string;
	let shortDescription: string;
	let longDescription: string;

	// Logic
	let waiting = false;

	function tryCreateGame() {
		if (gameTitle && shortDescription) {
			waiting = true;
			invoke('plugin:game|create_game', { gameTitle, shortDescription, longDescription })
				.then(() => {})
				.catch((e) => console.log(e))
				.finally(() => (waiting = false));
		}
	}
</script>

<main style="container">
	<h1>Create New Game</h1>
	<label for="Game Title">Game Title</label>
	<input name="gameTitle" placeholder="Game Title" bind:value={gameTitle} disabled={waiting} />

	<label for="Short Description">Short Description</label>
	<input
		name="shortDescription"
		placeholder="Short Description"
		bind:value={shortDescription}
		disabled={waiting}
	/>

	<label for="Long Description">Long Description</label>
	<input
		name="longDescription"
		placeholder="Long Description"
		bind:value={longDescription}
		disabled={waiting}
	/>

	<BackButton />
	<button disabled={waiting} on:click={tryCreateGame}>Create</button>
</main>
