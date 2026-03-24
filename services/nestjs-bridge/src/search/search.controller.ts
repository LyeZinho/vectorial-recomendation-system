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
}
