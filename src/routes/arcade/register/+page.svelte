<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<main class="container">
	<label>
		Username
		<input name="username" placeholder="Username" />
	</label>

	<label for="Password">Password</label>
	<input type="password" id="password" placeholder="Password" />

	<label for="Password Confirmation">Password</label>
	<input type="password" id="passwordConfirm" placeholder="Password Confirmation" />

	<label for="Email">Email</label>
	<input type="email" id="email" placeholder="Email" />

	<label for="Email Confirmation">Email Confirmation</label>
	<input type="email" id="emailConfirm" placeholder="Email Confirmation" />

	<a href="/arcade" role="button">Back</a>
	<button>Create</button>
</main>
