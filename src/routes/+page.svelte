<script lang="ts">
	import type { PageData } from './$types';
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';

	export let data: PageData;

	let username: String = '';
	let password: String = '';
	let waiting: boolean = false;

	function tryLogin() {
		waiting = true;
		invoke('plugin:auth|try_login', { username, password })
			.catch((err) => console.log(err))
			.finally(() => (waiting = false));
	}

	function clickedRegister() {
		goto('./register');
	}
</script>

<main class="container">
	<label for="Username">Username</label>
	<input disabled={waiting} name="username" placeholder="Username" bind:value={username} />

	<label for="Password">Password</label>
	<input
		disabled={waiting}
		type="password"
		id="password"
		placeholder="Password"
		bind:value={password}
	/>

	<button disabled={waiting} on:click={tryLogin}>Login</button>
	<button disabled={waiting} on:click={clickedRegister}>Register</button>
</main>
