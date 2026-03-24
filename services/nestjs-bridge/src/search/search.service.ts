import { Injectable } from '@nestjs/common';
import axios from 'axios';

interface SearchParams {
  query?: string;
  genre?: string;
  rating_min?: number;
  year?: number;
  offset?: number;
  limit?: number;
}

@Injectable()
export class SearchService {
  private readonly rustApiUrl = process.env.RUST_API_URL || 'http://localhost:3000';

  async search(params: SearchParams) {
    try {
      const queryParams = new URLSearchParams();

      if (params.query) queryParams.append('q', params.query);
      if (params.genre) queryParams.append('genre', params.genre);
      if (params.rating_min) queryParams.append('rating_min', params.rating_min.toString());
      if (params.year) queryParams.append('year', params.year.toString());
      if (params.offset !== undefined) queryParams.append('offset', params.offset.toString());
      if (params.limit !== undefined) queryParams.append('limit', params.limit.toString());

      const url = `${this.rustApiUrl}/api/search?${queryParams.toString()}`;
      const response = await axios.get(url);

      return {
        results: response.data.results || [],
        total: response.data.total || 0,
        offset: params.offset || 0,
        limit: params.limit || 20,
      };
    } catch (error: any) {
      throw new Error(`Search failed: ${error.message}`);
    }
  }
}
