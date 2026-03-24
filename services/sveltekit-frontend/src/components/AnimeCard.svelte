<script lang="ts">
  import { addToWatchlist, removeFromWatchlist } from '$lib/api';

  export let anime: any;
  export let onWatchlistChange: (anime: any) => void = () => {};

  let isInWatchlist = false;
  let isLoading = false;

  async function toggleWatchlist() {
    isLoading = true;
    try {
      if (isInWatchlist) {
        await removeFromWatchlist(anime.id);
        isInWatchlist = false;
      } else {
        await addToWatchlist(anime.id, 'watching');
        isInWatchlist = true;
      }
      onWatchlistChange(anime);
    } catch (error) {
      console.error('Error updating watchlist:', error);
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="card flex flex-col h-full">
  <div class="mb-4 bg-gray-800 h-48 flex items-center justify-center">
    {#if anime.poster_url}
      <img src={anime.poster_url} alt={anime.title} class="h-full object-cover" />
    {:else}
      <span class="text-gray-500">No Image</span>
    {/if}
  </div>

  <h3 class="text-lg font-black text-yellow-300 mb-2">{anime.title}</h3>

  <div class="text-sm text-purple-400 mb-2">
    {#if anime.score}
      <p>⭐ {anime.score}/10</p>
    {/if}
    {#if anime.genres}
      <p>{anime.genres.join(', ')}</p>
    {/if}
  </div>

  {#if anime.synopsis}
    <p class="text-sm text-gray-300 mb-4 flex-grow">{anime.synopsis.substring(0, 100)}...</p>
  {/if}

  <button
    on:click={toggleWatchlist}
    disabled={isLoading}
    class="mt-auto w-full font-bold py-2 transition"
    class:bg-yellow-300={isInWatchlist}
    class:text-black={isInWatchlist}
    class:bg-purple-600={!isInWatchlist}
    class:text-white={!isInWatchlist}
    class:opacity-50={isLoading}
  >
    {isLoading ? 'Loading...' : isInWatchlist ? '✓ In Watchlist' : '+ Add to Watchlist'}
  </button>
</div>
