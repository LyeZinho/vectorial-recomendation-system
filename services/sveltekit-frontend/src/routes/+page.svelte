<script>
  import { authStore } from './stores/auth';
  
  let isLoggedIn = false;
  authStore.subscribe(state => {
    isLoggedIn = state.isLoggedIn;
  });
</script>

<div class="min-h-screen flex flex-col items-center justify-center">
  <h1 class="text-6xl font-black mb-4 text-yellow-300">ANIME.VEC</h1>
  <p class="text-2xl mb-8 text-purple-400">Vectorial Recommendation Engine</p>
  
  {#if !isLoggedIn}
    <a href="/login" class="btn-primary text-lg">
      GET STARTED
    </a>
  {:else}
    <a href="/recommendations" class="btn-primary text-lg">
      VIEW RECOMMENDATIONS
    </a>
  {/if}
</div>
