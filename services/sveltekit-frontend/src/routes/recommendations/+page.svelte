<script lang="ts">
  import { onMount } from 'svelte';
  import { authStore } from '../../stores/auth';
  import { recommendationsStore } from '../../stores/recommendations';
  import AnimeCard from '../../components/AnimeCard.svelte';

  let user: any = null;
  let recommendations: any[] = [];
  let isLoading = false;
  let error: string | null = null;

  authStore.subscribe(state => {
    user = state.user;
  });

  recommendationsStore.subscribe(state => {
    recommendations = state.recommendations;
    isLoading = state.isLoading;
    error = state.error;
  });

  onMount(async () => {
    if (user?.id) {
      await recommendationsStore.fetchRecommendations(user.id);
    }
  });

  function handleWatchlistChange() {
    if (user?.id) {
      recommendationsStore.fetchRecommendations(user.id);
    }
  }
</script>

<div>
  <h1 class="text-4xl font-black mb-6 text-yellow-300">Your Recommendations</h1>
  
  {#if user}
    <p class="text-lg mb-6">Welcome, <span class="text-purple-400 font-bold">{user.username}</span>!</p>

    {#if isLoading}
      <div class="text-center text-purple-400 text-xl font-bold">
        Loading recommendations...
      </div>
    {:else if error}
      <div class="bg-red-900 border-2 border-red-400 p-4 text-red-100 mb-6">
        {error}
      </div>
    {:else if recommendations.length > 0}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {#each recommendations as anime (anime.id)}
          <AnimeCard {anime} onWatchlistChange={handleWatchlistChange} />
        {/each}
      </div>
    {:else}
      <div class="text-center text-gray-400 text-lg">
        No recommendations available at this time.
      </div>
    {/if}
  {:else}
    <p class="text-gray-400">Please log in to see recommendations</p>
  {/if}
</div>
