import { Injectable } from '@nestjs/common';
import axios from 'axios';

@Injectable()
export class RecommendationsService {
  private readonly rustApiUrl = process.env.RUST_API_URL || 'http://localhost:3000';

  async getRecommendations(userId: string) {
    try {
      const response = await axios.get(
        `${this.rustApiUrl}/api/recommendations/${userId}`,
        {
          headers: {
            'Authorization': `Bearer ${process.env.JWT_SECRET || 'dev-secret'}`,
          },
        },
      );
      return response.data;
    } catch (error) {
      console.error('Error fetching recommendations from Rust API:', error);
      return { recommendations: [], error: 'Failed to fetch recommendations' };
    }
  }
}
