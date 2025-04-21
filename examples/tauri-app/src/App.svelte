<script>
    import Greet from './lib/Greet.svelte'
    import { authenticate } from 'tauri-plugin-web-auth-api'

	let response = ''

	function updateResponse(returnValue) {
		response += `[${new Date().toLocaleTimeString()}] ` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}

	async function sign_in_with_google() {
        const scheme = "com.googleusercontent.apps.807645982538-7ib8rr1tkjkeb3vr0u1lk7gs6ip3b0jl"
        const url = new URL("https://accounts.google.com/o/oauth2/v2/auth")
        url.searchParams.set("response_type", "code")
        url.searchParams.set("client_id", "807645982538-7ib8rr1tkjkeb3vr0u1lk7gs6ip3b0jl.apps.googleusercontent.com")
        url.searchParams.set("redirect_uri", scheme+":/")
        url.searchParams.set("scope", "email")

		const res = await authenticate({
            url: url.toString(),
            callbackScheme: scheme,
        })
            .catch(updateResponse)

        if (!res) return

        const callback_url = new URL(res.callbackUrl)
        updateResponse(`Got code: ${callback_url.searchParams.get("code")}`)   
	}
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

  <div class="row">
    <Greet />
  </div>

  <div>
    <button on:click={sign_in_with_google}>Sign In With Google</button>
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
