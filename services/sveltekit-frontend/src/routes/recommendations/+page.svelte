<script>
  import { authStore } from '../../stores/auth';
  
  let user = null;
  authStore.subscribe(state => {
    user = state.user;
  });
</script>

<div>
  <h1 class="text-4xl font-black mb-6 text-yellow-300">Your Recommendations</h1>
  
  {#if user}
    <p class="text-lg mb-4">Welcome, <span class="text-purple-400 font-bold">{user.username}</span>!</p>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <!-- Recommendations will go here -->
      <div class="card">
        <p>Loading recommendations from Rust engine...</p>
      </div>
    </div>
  {:else}
    <p>Please log in to see recommendations</p>
  {/if}
</div>
