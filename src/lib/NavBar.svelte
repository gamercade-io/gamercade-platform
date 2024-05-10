<script>
	import { goto } from '$app/navigation';

	function gotoArcade() {
		goto('/');
	}
	function gotoLibrary() {
		goto('/library');
	}
	function gotoSettings() {
		goto('/settings');
	}
	function gotoDashboard() {
		goto('/dashboard');
	}
</script>

<button on:click={gotoArcade}>Arcade</button>
<button on:click={gotoLibrary}>Library</button>
<button on:click={gotoSettings}>Settings</button>
<button on:click={gotoDashboard}>Creator Dashboard</button>
