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

	<a href="/arcade/top" role="button">Login</a>
	<a href="/arcade/register" role="button">Register</a>
</main>
