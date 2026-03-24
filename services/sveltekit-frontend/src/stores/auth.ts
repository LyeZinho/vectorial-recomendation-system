import { writable } from 'svelte/store';

interface AuthState {
  user: any | null;
  isLoggedIn: boolean;
  token: string | null;
}

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>({
    user: null,
    isLoggedIn: false,
    token: null,
  });

  return {
    subscribe,
    login: (user: any, token: string) => {
      localStorage.setItem('access_token', token);
      set({ user, isLoggedIn: true, token });
    },
    logout: () => {
      localStorage.removeItem('access_token');
      set({ user: null, isLoggedIn: false, token: null });
    },
    setUser: (user: any) => update(state => ({ ...state, user })),
  };
}

export const authStore = createAuthStore();
