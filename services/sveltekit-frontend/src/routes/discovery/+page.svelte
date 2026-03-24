<script lang="ts">
  import { onMount } from 'svelte';
  import { authStore } from '../../stores/auth';
  import SearchBar from '../../components/SearchBar.svelte';
  import AnimeCard from '../../components/AnimeCard.svelte';

  let user: any = null;
  let searchResults: any[] = [];
  let isLoading = false;
  let error: string | null = null;
  let hasSearched = false;

  authStore.subscribe(state => {
    user = state.user;
  });

  async function handleSearch(query: string) {
    isLoading = true;
    error = null;
    hasSearched = true;
    searchResults = [];

    try {
      const params = new URLSearchParams({
        q: query,
        offset: '0',
        limit: '20',
      });

      const response = await fetch(
        `http://localhost:3001/search?${params.toString()}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem('access_token')}`,
          },
        }
      );

      if (!response.ok) {
        throw new Error(`Search failed: ${response.status}`);
      }

      const data = await response.json();
      searchResults = data.results || [];
    } catch (err: any) {
      error = err.message || 'Search failed. Please try again.';
    } finally {
      isLoading = false;
    }
  }

  function handleWatchlistChange() {
    // Results stay on screen, watchlist updated in background
  }
</script>

<div>
  <h1 class="text-4xl font-black mb-6 text-yellow-300">Discover Anime</h1>

  {#if user}
    <div class="mb-8">
      <SearchBar {isLoading} onSearch={handleSearch} />
    </div>

    {#if error}
      <div class="bg-red-900 border-2 border-red-400 p-4 text-red-100 mb-6">
        {error}
      </div>
    {/if}

    {#if isLoading}
      <div class="text-center text-purple-400 text-xl font-bold">
        Searching...
      </div>
    {:else if hasSearched && searchResults.length === 0}
      <div class="text-center text-gray-400 text-lg">
        No results found. Try a different search.
      </div>
    {:else if searchResults.length > 0}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {#each searchResults as anime (anime.id)}
          <AnimeCard {anime} onWatchlistChange={handleWatchlistChange} />
        {/each}
      </div>
    {:else}
      <div class="text-center text-gray-400 text-lg">
        Start searching to discover anime!
      </div>
    {/if}
  {:else}
    <p class="text-gray-400">Please log in to search anime</p>
  {/if}
</div>
