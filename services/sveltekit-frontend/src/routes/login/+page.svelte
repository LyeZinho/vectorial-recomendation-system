<script lang="ts">
  import { goto } from '$app/navigation';
  import { login, register } from '$lib/api';
  import { authStore } from '../../stores/auth';

  let isLogin = true;
  let email = '';
  let username = '';
  let password = '';
  let error = '';
  let loading = false;

  async function handleSubmit() {
    loading = true;
    error = '';

    try {
      let result;
      if (isLogin) {
        result = await login(email, password);
      } else {
        result = await register(email, username, password);
      }

      authStore.login(result.user, result.access_token);
      await goto('/recommendations');
    } catch (err: any) {
      error = err.message || 'Authentication failed';
    } finally {
      loading = false;
    }
  }

  function toggleMode() {
    isLogin = !isLogin;
    error = '';
  }
</script>

<div class="min-h-screen bg-black text-white flex items-center justify-center p-4">
  <div class="w-full max-w-md">
    <h1 class="text-4xl font-black text-center mb-8 text-yellow-300">
      ANIME.VEC
    </h1>

    <form on:submit|preventDefault={handleSubmit} class="space-y-4 border-2 border-yellow-300 p-8">
      {#if !isLogin}
        <div>
          <label class="block text-sm font-bold mb-2">Username</label>
          <input
            type="text"
            bind:value={username}
            required
            class="w-full bg-gray-900 border-2 border-purple-400 p-3 text-white font-bold"
          />
        </div>
      {/if}

      <div>
        <label class="block text-sm font-bold mb-2">Email</label>
        <input
          type="email"
          bind:value={email}
          required
          class="w-full bg-gray-900 border-2 border-purple-400 p-3 text-white font-bold"
        />
      </div>

      <div>
        <label class="block text-sm font-bold mb-2">Password</label>
        <input
          type="password"
          bind:value={password}
          required
          class="w-full bg-gray-900 border-2 border-purple-400 p-3 text-white font-bold"
        />
      </div>

      {#if error}
        <div class="bg-red-900 border-2 border-red-400 p-3 text-red-100">
          {error}
        </div>
      {/if}

      <button
        type="submit"
        disabled={loading}
        class="w-full bg-yellow-300 text-black font-bold py-3 hover:bg-yellow-200 disabled:opacity-50"
      >
        {loading ? 'Loading...' : isLogin ? 'Login' : 'Register'}
      </button>

      <button
        type="button"
        on:click={toggleMode}
        class="w-full text-purple-400 font-bold py-2 hover:text-purple-300"
      >
        {isLogin ? 'Need an account? Register' : 'Already have an account? Login'}
      </button>
    </form>
  </div>
</div>
