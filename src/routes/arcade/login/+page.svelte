<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<label>
	Username
	<input name="username" placeholder="Username" />
</label>

<label for="Password">Password</label>
<input type="password" id="password" placeholder="Password" />

<button>Login</button>
<a href="/arcade/register" role="button">Register</a>
