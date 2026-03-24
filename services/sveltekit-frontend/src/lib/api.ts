const API_URL = process.env.PUBLIC_API_URL || 'http://localhost:3001';

export async function apiCall(endpoint: string, options: any = {}) {
  const token = typeof window !== 'undefined' ? localStorage.getItem('access_token') : null;

  const headers: any = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const response = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    throw new Error(`API error: ${response.status}`);
  }

  return response.json();
}

export async function login(email: string, password: string) {
  return apiCall('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
  });
}

export async function register(email: string, username: string, password: string) {
  return apiCall('/auth/register', {
    method: 'POST',
    body: JSON.stringify({ email, username, password }),
  });
}

export async function getMe() {
  return apiCall('/users/me');
}

export async function getRecommendations(userId: string) {
  return apiCall(`/recommendations/${userId}`);
}

export async function addToWatchlist(animeId: number, status: string = 'watching') {
  return apiCall('/watchlist', {
    method: 'POST',
    body: JSON.stringify({ anime_id: animeId, status }),
  });
}

export async function removeFromWatchlist(id: string) {
  return apiCall(`/watchlist/${id}`, {
    method: 'DELETE',
  });
}

export async function getWatchlist(status?: string) {
  const query = status ? `?status=${status}` : '';
  return apiCall(`/watchlist${query}`);
}

export async function updateWatchlistStatus(id: string, status: string) {
  return apiCall(`/watchlist/${id}`, {
    method: 'PUT',
    body: JSON.stringify({ status }),
  });
}
