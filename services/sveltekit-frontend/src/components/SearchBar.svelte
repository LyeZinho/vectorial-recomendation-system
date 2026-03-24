<script lang="ts">
  export let onSearch: (query: string) => void = () => {};
  export let loading = false;

  let searchQuery = '';

  function handleSubmit() {
    if (searchQuery.trim()) {
      onSearch(searchQuery);
    }
  }

  function handleClear() {
    searchQuery = '';
  }
</script>

<div class="mb-6 flex gap-2">
  <input
    type="text"
    bind:value={searchQuery}
    placeholder="Search anime..."
    on:keydown={e => e.key === 'Enter' && handleSubmit()}
    disabled={loading}
    class="flex-1 bg-gray-900 border-2 border-purple-600 text-white p-3 font-bold placeholder-gray-500"
  />
  <button
    on:click={handleSubmit}
    disabled={loading || !searchQuery.trim()}
    class="bg-yellow-300 text-black font-bold px-6 py-3 hover:bg-yellow-200 disabled:opacity-50 transition"
  >
    {loading ? 'Searching...' : 'Search'}
  </button>
  <button
    on:click={handleClear}
    disabled={loading}
    class="bg-purple-600 text-white font-bold px-4 py-3 hover:bg-purple-500 disabled:opacity-50 transition"
  >
    Clear
  </button>
</div>
