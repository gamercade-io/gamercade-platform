<script lang="ts">
	import { goto } from '$app/navigation';
	export let target: string;
	export let text: string;
</script>

<button on:click={() => goto(target)}>{text}</button>
