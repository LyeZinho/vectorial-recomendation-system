import { writable } from 'svelte/store';
import { getRecommendations } from '$lib/api';

interface RecommendationsState {
  recommendations: any[];
  isLoading: boolean;
  error: string | null;
  userId: string | null;
}

function createRecommendationsStore() {
  const { subscribe, set, update } = writable<RecommendationsState>({
    recommendations: [],
    isLoading: false,
    error: null,
    userId: null,
  });

  return {
    subscribe,
    fetchRecommendations: async (userId: string) => {
      set({
        recommendations: [],
        isLoading: true,
        error: null,
        userId,
      });

      try {
        const result = await getRecommendations(userId);
        set({
          recommendations: result.recommendations || [],
          isLoading: false,
          error: result.error || null,
          userId,
        });
      } catch (error: any) {
        set({
          recommendations: [],
          isLoading: false,
          error: error.message || 'Failed to fetch recommendations',
          userId,
        });
      }
    },
    clear: () => {
      set({
        recommendations: [],
        isLoading: false,
        error: null,
        userId: null,
      });
    },
  };
}

export const recommendationsStore = createRecommendationsStore();
