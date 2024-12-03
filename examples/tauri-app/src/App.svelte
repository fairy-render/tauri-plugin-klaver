<script>
  import { open, ping } from '@fairy-render/plugin-klaver'
  import { listen } from '@tauri-apps/api/event';
  import Greet from './lib/Greet.svelte'

	let response = ''
  let textArea;

  let vm;

	function updateResponse(returnValue) {
		response += `[${new Date().toLocaleTimeString()}] ` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}



	async function _ping() {
		if (!vm) {
      vm = await open(".")
      vm.listen((level, msg) => {
        response += `${level}: ${msg} <br/>`
      });
    }

    const val = await vm.eval('main',textArea.value)

    console.log(await vm.typings())

  
	}

 

</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <textarea bind:this={textArea}></textarea>
  </div>

  <div>
    <button on:click="{_ping}">Ping</button>
    <div>{@html response}</div>
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
