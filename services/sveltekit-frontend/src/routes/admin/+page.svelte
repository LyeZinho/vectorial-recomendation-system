<script lang="ts">
  import { onMount } from 'svelte';
  import { authStore } from '../../stores/auth';
  import { goto } from '$app/navigation';

  let user: any = null;
  let isAdmin = false;
  let stats: any = null;
  let users: any[] = [];
  let isLoading = true;
  let error: string | null = null;
  let currentPage = 0;
  let totalUsers = 0;
  const usersPerPage = 25;

  authStore.subscribe(state => {
    user = state.user;
    isAdmin = state.user?.role === 'admin';
  });

  async function loadStats() {
    try {
      const response = await fetch('http://localhost:3001/admin/stats', {
        headers: {
          Authorization: `Bearer ${localStorage.getItem('access_token')}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to load stats: ${response.status}`);
      }

      stats = await response.json();
    } catch (err: any) {
      error = err.message || 'Failed to load stats';
    }
  }

  async function loadUsers(page: number = 0) {
    try {
      const offset = page * usersPerPage;
      const response = await fetch(
        `http://localhost:3001/admin/users?limit=${usersPerPage}&offset=${offset}`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem('access_token')}`,
          },
        }
      );

      if (!response.ok) {
        throw new Error(`Failed to load users: ${response.status}`);
      }

      const data = await response.json();
      users = data.users || [];
      totalUsers = data.total || 0;
      currentPage = page;
    } catch (err: any) {
      error = err.message || 'Failed to load users';
    }
  }

  async function updateUserRole(userId: string, newRole: 'user' | 'admin') {
    try {
      const response = await fetch(`http://localhost:3001/admin/users/${userId}/role`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${localStorage.getItem('access_token')}`,
        },
        body: JSON.stringify({ role: newRole }),
      });

      if (!response.ok) {
        throw new Error(`Failed to update user: ${response.status}`);
      }

      await loadUsers(currentPage);
    } catch (err: any) {
      error = err.message || 'Failed to update user role';
    }
  }

  async function deleteUser(userId: string) {
    if (!confirm('Are you sure you want to delete this user?')) return;

    try {
      const response = await fetch(`http://localhost:3001/admin/users/${userId}`, {
        method: 'DELETE',
        headers: {
          Authorization: `Bearer ${localStorage.getItem('access_token')}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to delete user: ${response.status}`);
      }

      await loadUsers(currentPage);
    } catch (err: any) {
      error = err.message || 'Failed to delete user';
    }
  }

  onMount(async () => {
    if (!isAdmin) {
      await goto('/');
      return;
    }

    isLoading = true;
    await loadStats();
    await loadUsers(0);
    isLoading = false;
  });

  function nextPage() {
    const maxPage = Math.ceil(totalUsers / usersPerPage) - 1;
    if (currentPage < maxPage) {
      loadUsers(currentPage + 1);
    }
  }

  function prevPage() {
    if (currentPage > 0) {
      loadUsers(currentPage - 1);
    }
  }
</script>

<div>
  <h1 class="text-4xl font-black mb-6 text-yellow-300">Admin Dashboard</h1>

  {#if !isAdmin}
    <div class="bg-red-900 border-2 border-red-400 p-4 text-red-100">
      Access denied. Admin role required.
    </div>
  {:else if isLoading}
    <div class="text-center text-purple-400 text-xl font-bold">Loading...</div>
  {:else if error}
    <div class="bg-red-900 border-2 border-red-400 p-4 text-red-100 mb-6">
      {error}
    </div>
  {:else}
    <!-- Stats Section -->
    <div class="mb-8">
      <h2 class="text-2xl font-bold text-yellow-300 mb-4">System Statistics</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {#if stats}
          <div class="bg-gray-900 border-2 border-purple-600 p-4">
            <p class="text-gray-400 text-sm font-bold mb-2">Total Users</p>
            <p class="text-3xl font-black text-yellow-300">{stats.users?.total || 0}</p>
          </div>

          <div class="bg-gray-900 border-2 border-purple-600 p-4">
            <p class="text-gray-400 text-sm font-bold mb-2">Admin Users</p>
            <p class="text-3xl font-black text-yellow-300">{stats.users?.admins || 0}</p>
          </div>

          <div class="bg-gray-900 border-2 border-purple-600 p-4">
            <p class="text-gray-400 text-sm font-bold mb-2">Total Anime</p>
            <p class="text-3xl font-black text-yellow-300">{stats.anime?.total || 0}</p>
          </div>

          <div class="bg-gray-900 border-2 border-purple-600 p-4">
            <p class="text-gray-400 text-sm font-bold mb-2">Recommendations</p>
            <p class="text-3xl font-black text-yellow-300">{stats.recommendations?.total || 0}</p>
          </div>
        {/if}
      </div>
    </div>

    <!-- Users Management Section -->
    <div>
      <h2 class="text-2xl font-bold text-yellow-300 mb-4">User Management</h2>
      <p class="text-gray-400 mb-4">
        Showing {currentPage * usersPerPage + 1}–{Math.min((currentPage + 1) * usersPerPage, totalUsers)} of {totalUsers} users
      </p>

      <div class="overflow-x-auto border-2 border-purple-600 mb-6">
        <table class="w-full">
          <thead>
            <tr class="bg-purple-900">
              <th class="p-3 text-left font-bold text-yellow-300">Email</th>
              <th class="p-3 text-left font-bold text-yellow-300">Username</th>
              <th class="p-3 text-left font-bold text-yellow-300">Role</th>
              <th class="p-3 text-left font-bold text-yellow-300">Created</th>
              <th class="p-3 text-left font-bold text-yellow-300">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each users as userItem (userItem.id)}
              <tr class="border-t-2 border-purple-600 hover:bg-gray-800">
                <td class="p-3 text-white">{userItem.email}</td>
                <td class="p-3 text-white">{userItem.username}</td>
                <td class="p-3">
                  <span
                    class="px-2 py-1 font-bold text-sm"
                    class:bg-yellow-300={userItem.role === 'admin'}
                    class:text-black={userItem.role === 'admin'}
                    class:bg-purple-600={userItem.role === 'user'}
                    class:text-white={userItem.role === 'user'}
                  >
                    {userItem.role}
                  </span>
                </td>
                <td class="p-3 text-gray-400 text-sm">
                  {new Date(userItem.created_at).toLocaleDateString()}
                </td>
                <td class="p-3 flex gap-2">
                  {#if userItem.role === 'user'}
                    <button
                      on:click={() => updateUserRole(userItem.id, 'admin')}
                      class="bg-yellow-300 text-black px-2 py-1 font-bold text-sm hover:bg-yellow-200 transition"
                    >
                      Make Admin
                    </button>
                  {:else}
                    <button
                      on:click={() => updateUserRole(userItem.id, 'user')}
                      class="bg-gray-600 text-white px-2 py-1 font-bold text-sm hover:bg-gray-500 transition"
                    >
                      Remove Admin
                    </button>
                  {/if}

                  <button
                    on:click={() => deleteUser(userItem.id)}
                    class="bg-red-700 text-white px-2 py-1 font-bold text-sm hover:bg-red-600 transition"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <div class="flex gap-2 justify-center">
        <button
          on:click={prevPage}
          disabled={currentPage === 0}
          class="bg-purple-600 text-white font-bold px-4 py-2 hover:bg-purple-500 disabled:opacity-50 transition"
        >
          ← Previous
        </button>
        <button
          on:click={nextPage}
          disabled={currentPage >= Math.ceil(totalUsers / usersPerPage) - 1}
          class="bg-purple-600 text-white font-bold px-4 py-2 hover:bg-purple-500 disabled:opacity-50 transition"
        >
          Next →
        </button>
      </div>
    </div>
  {/if}
</div>
