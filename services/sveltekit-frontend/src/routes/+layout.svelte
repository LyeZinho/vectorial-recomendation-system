<script lang="ts">
  import { goto } from '$app/navigation';
  import { authStore } from '../stores/auth';
  import '../app.css';

  let isLoggedIn = false;
  let isAdmin = false;

  authStore.subscribe(state => {
    isLoggedIn = state.isLoggedIn;
    isAdmin = state.user?.role === 'admin';
  });

  async function handleLogout() {
    authStore.logout();
    await goto('/');
  }
</script>

<div class="min-h-screen bg-black text-white">
  <nav class="border-b-2 border-yellow-300 p-4 flex justify-between items-center">
    <a href="/" class="text-2xl font-black text-yellow-300">ANIME.VEC</a>
    
    <div class="flex gap-4 items-center">
      {#if isLoggedIn}
        <a href="/discovery" class="font-bold hover:text-yellow-300">Discovery</a>
        <a href="/recommendations" class="font-bold hover:text-yellow-300">Recommendations</a>
        <a href="/profile" class="font-bold hover:text-yellow-300">Profile</a>
        {#if isAdmin}
          <a href="/admin" class="font-bold text-yellow-300 hover:underline">Admin</a>
        {/if}
        <button
          on:click={handleLogout}
          class="bg-purple-600 px-4 py-2 font-bold hover:bg-purple-500"
        >
          Logout
        </button>
      {:else}
        <a href="/login" class="btn-primary">Login</a>
      {/if}
    </div>
  </nav>

  <main class="p-4">
    <slot />
  </main>
</div>
