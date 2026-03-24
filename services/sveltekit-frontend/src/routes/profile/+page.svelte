<script lang="ts">
  import { onMount } from 'svelte';
  import { authStore } from '../../stores/auth';
  import { getWatchlist, updateWatchlistStatus, removeFromWatchlist } from '$lib/api';

  let user: any = null;
  let watchlist: any[] = [];
  let isLoading = false;
  let error: string | null = null;
  let selectedStatus: string = 'all';

  const statuses = ['watching', 'completed', 'dropped', 'planned'];

  authStore.subscribe(state => {
    user = state.user;
  });

  async function loadWatchlist() {
    isLoading = true;
    error = null;
    try {
      const status = selectedStatus === 'all' ? undefined : selectedStatus;
      const data = await getWatchlist(status);
      watchlist = data.watchlist || [];
    } catch (err: any) {
      error = err.message || 'Failed to load watchlist';
    } finally {
      isLoading = false;
    }
  }

  async function handleStatusChange(watchlistId: string, newStatus: string) {
    try {
      await updateWatchlistStatus(watchlistId, newStatus);
      await loadWatchlist();
    } catch (err: any) {
      error = err.message || 'Failed to update status';
    }
  }

  async function handleRemove(watchlistId: string) {
    try {
      await removeFromWatchlist(watchlistId);
      await loadWatchlist();
    } catch (err: any) {
      error = err.message || 'Failed to remove from watchlist';
    }
  }

  onMount(async () => {
    if (user?.id) {
      await loadWatchlist();
    }
  });
</script>

<div class="max-w-6xl mx-auto">
  <h1 class="text-4xl font-black mb-6 text-yellow-300">Profile</h1>

  {#if user}
    <!-- User Info Section -->
    <div class="bg-gray-900 border-2 border-purple-600 p-8 mb-8">
      <h2 class="text-2xl font-bold text-yellow-300 mb-4">User Information</h2>
      <div class="space-y-2">
        <p><span class="text-purple-400 font-bold">Username:</span> {user.username}</p>
        <p><span class="text-purple-400 font-bold">Email:</span> {user.email}</p>
        {#if user.profile?.bio}
          <p><span class="text-purple-400 font-bold">Bio:</span> {user.profile.bio}</p>
        {/if}
        {#if user.profile?.genre}
          <p><span class="text-purple-400 font-bold">Favorite Genre:</span> {user.profile.genre}</p>
        {/if}
      </div>
    </div>

    <!-- Watchlist Section -->
    <div>
      <h2 class="text-2xl font-bold text-yellow-300 mb-4">Watchlist</h2>

      <!-- Status Filter -->
      <div class="mb-6 flex gap-2 flex-wrap">
        <button
          on:click={() => {
            selectedStatus = 'all';
            loadWatchlist();
          }}
          class="px-4 py-2 font-bold transition"
          class:bg-yellow-300={selectedStatus === 'all'}
          class:text-black={selectedStatus === 'all'}
          class:bg-purple-600={selectedStatus !== 'all'}
          class:text-white={selectedStatus !== 'all'}
        >
          All
        </button>
        {#each statuses as status}
          <button
            on:click={() => {
              selectedStatus = status;
              loadWatchlist();
            }}
            class="px-4 py-2 font-bold transition capitalize"
            class:bg-yellow-300={selectedStatus === status}
            class:text-black={selectedStatus === status}
            class:bg-purple-600={selectedStatus !== status}
            class:text-white={selectedStatus !== status}
          >
            {status}
          </button>
        {/each}
      </div>

      {#if error}
        <div class="bg-red-900 border-2 border-red-400 p-4 text-red-100 mb-6">
          {error}
        </div>
      {/if}

      {#if isLoading}
        <div class="text-center text-purple-400 text-xl font-bold">
          Loading watchlist...
        </div>
      {:else if watchlist.length > 0}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each watchlist as item (item.id)}
            <div class="bg-gray-800 border-2 border-purple-600 p-4">
              <h3 class="text-lg font-black text-yellow-300 mb-2">{item.anime?.title || 'Unknown'}</h3>

              <div class="mb-4 text-sm text-gray-300">
                {#if item.anime?.score}
                  <p>⭐ {item.anime.score}/10</p>
                {/if}
                {#if item.anime?.genres}
                  <p class="text-purple-400">{item.anime.genres.join(', ')}</p>
                {/if}
              </div>

              <div class="mb-4">
                <label class="text-sm text-purple-400 font-bold block mb-2">Status:</label>
                <select
                  value={item.status}
                  on:change={e => handleStatusChange(item.id, e.currentTarget.value)}
                  class="w-full bg-gray-900 border-2 border-purple-600 text-white p-2 font-bold"
                >
                  {#each statuses as status}
                    <option value={status} class="bg-gray-900">
                      {status.charAt(0).toUpperCase() + status.slice(1)}
                    </option>
                  {/each}
                </select>
              </div>

              <button
                on:click={() => handleRemove(item.id)}
                class="w-full bg-red-700 hover:bg-red-600 text-white font-bold py-2 transition"
              >
                Remove
              </button>
            </div>
          {/each}
        </div>
      {:else}
        <div class="text-center text-gray-400 text-lg">
          No anime in your watchlist yet. Go to{' '}
          <a href="/recommendations" class="text-yellow-300 font-bold hover:underline">recommendations</a>
          {' '}to add some!
        </div>
      {/if}
    </div>
  {:else}
    <p class="text-gray-400">Please log in to see your profile</p>
  {/if}
</div>

<style global>
  :global(body) {
    @apply bg-black;
  }
</style>
