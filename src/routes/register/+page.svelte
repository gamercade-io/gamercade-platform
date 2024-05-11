<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from '../register/$types';
	import { invoke } from '@tauri-apps/api';

	export let data: PageData;

	// Fields
	let username: String;
	let password: String;
	let passwordConfirm: String;
	let email: String;
	let emailConfirm: String;

	// Logic
	let usernameInvalid: boolean | null = null;
	let passwordInvalid: boolean | null = null;
	let emailInvalid: boolean | null = null;
	let waiting: boolean = false;
	// TOOD: Show a modal with the result of the sign up request

	function resetUsername() {
		usernameInvalid = null;
	}

	function resetEmail() {
		emailInvalid = null;
	}

	function resetPassword() {
		passwordInvalid = null;
	}

	function trySignUp() {
		if (username) {
			usernameInvalid = false;
		} else {
			usernameInvalid = true;
		}

		if (!password || !passwordConfirm || password !== passwordConfirm) {
			passwordInvalid = true;
		} else {
			passwordInvalid = false;
		}

		if (!email || !emailConfirm || email !== emailConfirm) {
			emailInvalid = true;
		} else {
			emailInvalid = false;
		}

		if (passwordInvalid === true || emailInvalid === true || usernameInvalid === true) {
			return;
		}

		waiting = true;
		invoke('plugin:auth|try_signup', { username, password, email })
			.catch((err) => console.log(err))
			.finally(() => (waiting = false));
	}
</script>

<main class="container">
	<label for="username">Username</label>
	<input
		id="username"
		disabled={waiting}
		aria-invalid={usernameInvalid}
		name="username"
		placeholder="Username"
		bind:value={username}
		on:keydown={resetUsername}
	/>
	{#if usernameInvalid === true}
		<small id="username-error-helper">Invalid Username.</small>
	{/if}

	<label for="password">Password</label>
	<input
		disabled={waiting}
		aria-invalid={passwordInvalid}
		type="password"
		id="password"
		placeholder="Password"
		bind:value={password}
		on:keydown={resetPassword}
	/>

	<label for="passwordConfirm">Password Confirmation</label>
	<input
		disabled={waiting}
		aria-invalid={passwordInvalid}
		aria-describedby="password-error-helper"
		type="password"
		id="passwordConfirm"
		placeholder="Password Confirmation"
		bind:value={passwordConfirm}
		on:keydown={resetPassword}
	/>
	{#if passwordInvalid === true}
		<small id="password-error-helper">Passwords do not match.</small>
	{/if}

	<label for="email">Email</label>
	<input
		disabled={waiting}
		aria-invalid={emailInvalid}
		type="email"
		id="email"
		placeholder="Email"
		bind:value={email}
		on:keydown={resetEmail}
	/>

	<label for="emailConfirm">Email Confirmation</label>

	<input
		disabled={waiting}
		aria-invalid={emailInvalid}
		aria-describedby="email-error-helper"
		type="email"
		id="emailConfirm"
		placeholder="Email Confirmation"
		bind:value={emailConfirm}
		on:keydown={resetEmail}
	/>
	{#if emailInvalid === true}
		<small id="email-error-helper">Emails do not match.</small>
	{/if}

	<BackButton />
	<button aria-busy={waiting} disabled={waiting} on:click={trySignUp}>Create</button>
</main>
