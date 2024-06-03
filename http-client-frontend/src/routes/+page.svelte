<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let content: string = "Send a Request"

	function send_request() {
		const url: string =  (<HTMLInputElement>document.getElementById("url")).value;
		console.log(url)
		 invoke("send_request", { url }).then(value => {
			 if (typeof value === 'string') {
				 content = JSON.stringify(JSON.parse(value), undefined, 2);
			 }
			 console.log(value);
		 }
		);
	}
</script>
<h1 class="h1 text-center m-5">Soxy</h1>
<div class="grid grid-cols-2 min-h-full">
	<div>
		<select name="request_method" id="request_method" class="text-black">
			<option value="POST">GET</option>
			<option value="POST">POST</option>
		</select>
		<label for="url">Enter url: </label>
		<input type="text" id="url" class="text-black">
		<button on:click={send_request}>Send Request</button>
	</div>
	<div class="border-white bg-slate-200  min-w-screen min-h-screen text-black m-5 rounded-2xl p-4 text-wrap">
		<pre class="whitespace-pre-line">{content}</pre>
	</div>
</div>


