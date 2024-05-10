<script lang="ts">
	import BackButton from '$lib/BackButton.svelte';
	import type { PageData } from '../register/$types';

	export let data: PageData;
</script>

<main class="container">
	<label for="Username">Username</label>
	<input name="username" placeholder="Username" />

	<label for="Password">Password</label>
	<input type="password" id="password" placeholder="Password" />

	<label for="Password Confirmation">Password</label>
	<input type="password" id="passwordConfirm" placeholder="Password Confirmation" />

	<label for="Email">Email</label>
	<input type="email" id="email" placeholder="Email" />

	<label for="Email Confirmation">Email Confirmation</label>
	<input type="email" id="emailConfirm" placeholder="Email Confirmation" />

	<BackButton />
	<button>Create</button>
</main>
