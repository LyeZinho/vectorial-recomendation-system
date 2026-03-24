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
  let genres: string[] = [];
  let years: number[] = [];
  let currentQuery = '';
  let selectedGenre = '';
  let selectedYear = '';
  let currentOffset = 0;
  let totalResults = 0;

  const limit = 20;

  authStore.subscribe(state => {
    user = state.user;
  });

  onMount(async () => {
    try {
      const genreRes = await fetch('http://localhost:3001/search/genres', {
        headers: {
          Authorization: `Bearer ${localStorage.getItem('access_token')}`,
        },
      });
      const genreData = await genreRes.json();
      genres = genreData.genres || [];

      const yearRes = await fetch('http://localhost:3001/search/years', {
        headers: {
          Authorization: `Bearer ${localStorage.getItem('access_token')}`,
        },
      });
      const yearData = await yearRes.json();
      years = yearData.years || [];
    } catch (err) {
      console.error('Failed to load filter options');
    }
  });

  async function performSearch(query: string, offset: number = 0) {
    isLoading = true;
    error = null;
    hasSearched = true;
    searchResults = [];
    currentOffset = offset;

    try {
      const params = new URLSearchParams({
        q: query,
        offset: offset.toString(),
        limit: limit.toString(),
      });

      if (selectedGenre) params.append('genre', selectedGenre);
      if (selectedYear) params.append('year', selectedYear);

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
      totalResults = data.total || 0;
    } catch (err: any) {
      error = err.message || 'Search failed. Please try again.';
    } finally {
      isLoading = false;
    }
  }

  function handleSearch(query: string) {
    currentQuery = query;
    performSearch(query, 0);
  }

  function handleFilterChange() {
    if (currentQuery) {
      performSearch(currentQuery, 0);
    }
  }

  function goToPreviousPage() {
    if (currentOffset > 0) {
      performSearch(currentQuery, currentOffset - limit);
    }
  }

  function goToNextPage() {
    if (currentOffset + limit < totalResults) {
      performSearch(currentQuery, currentOffset + limit);
    }
  }

  function handleWatchlistChange() {
    // Results stay on screen, watchlist updated in background
  }


<div>
  <h1 class="text-4xl font-black mb-6 text-yellow-300">Discover Anime</h1>

  {#if user}
    <div class="mb-8">
      <SearchBar {isLoading} onSearch={handleSearch} />
    </div>

    {#if hasSearched}
      <div class="mb-6 bg-gray-900 border-2 border-purple-600 p-4">
        <h3 class="text-lg font-bold text-yellow-300 mb-4">Filters</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-bold text-purple-400 mb-2">Genre</label>
            <select
              bind:value={selectedGenre}
              on:change={handleFilterChange}
              disabled={isLoading}
              class="w-full bg-gray-800 border-2 border-purple-600 text-white p-2 font-bold"
            >
              <option value="">All Genres</option>
              {#each genres as genre}
                <option value={genre}>{genre}</option>
              {/each}
            </select>
          </div>
          <div>
            <label class="block text-sm font-bold text-purple-400 mb-2">Year</label>
            <select
              bind:value={selectedYear}
              on:change={handleFilterChange}
              disabled={isLoading}
              class="w-full bg-gray-800 border-2 border-purple-600 text-white p-2 font-bold"
            >
              <option value="">All Years</option>
              {#each years as year}
                <option value={year}>{year}</option>
              {/each}
            </select>
          </div>
        </div>
      </div>
    {/if}

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
      <div>
        <p class="text-gray-400 mb-4">
          Showing {currentOffset + 1}–{Math.min(currentOffset + limit, totalResults)} of {totalResults} results
        </p>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
          {#each searchResults as anime (anime.id)}
            <AnimeCard {anime} onWatchlistChange={handleWatchlistChange} />
          {/each}
        </div>

        {#if totalResults > limit}
          <div class="flex gap-2 justify-center">
            <button
              on:click={goToPreviousPage}
              disabled={currentOffset === 0 || isLoading}
              class="bg-purple-600 text-white font-bold px-4 py-2 hover:bg-purple-500 disabled:opacity-50 transition"
            >
              ← Previous
            </button>
            <button
              on:click={goToNextPage}
              disabled={currentOffset + limit >= totalResults || isLoading}
              class="bg-purple-600 text-white font-bold px-4 py-2 hover:bg-purple-500 disabled:opacity-50 transition"
            >
              Next →
            </button>
          </div>
        {/if}
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
