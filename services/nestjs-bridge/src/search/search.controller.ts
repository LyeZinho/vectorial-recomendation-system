import { Controller, Get, Query, UseGuards } from '@nestjs/common';
import { SearchService } from './search.service';
import { JwtAuthGuard } from '../auth/jwt-auth.guard';

@Controller('search')
export class SearchController {
  constructor(private readonly searchService: SearchService) {}

  @Get()
  @UseGuards(JwtAuthGuard)
  async search(
    @Query('q') query?: string,
    @Query('genre') genre?: string,
    @Query('rating_min') rating_min?: string,
    @Query('year') year?: string,
    @Query('offset') offset?: string,
    @Query('limit') limit?: string,
  ) {
    return this.searchService.search({
      query,
      genre,
      rating_min: rating_min ? parseInt(rating_min) : undefined,
      year: year ? parseInt(year) : undefined,
      offset: offset ? parseInt(offset) : 0,
      limit: limit ? parseInt(limit) : 20,
    });
  }

  @Get('genres')
  @UseGuards(JwtAuthGuard)
  async getGenres() {
    return {
      genres: [
        'Action',
        'Adventure',
        'Comedy',
        'Drama',
        'Fantasy',
        'Horror',
        'Psychological',
        'Romance',
        'Sci-Fi',
        'Slice of Life',
        'Sports',
        'Thriller',
      ],
    };
  }

  @Get('years')
  @UseGuards(JwtAuthGuard)
  async getYears() {
    const currentYear = new Date().getFullYear();
    const years = [];
    for (let i = currentYear; i >= 1970; i--) {
      years.push(i);
    }
    return { years };
  }
}
