import { Controller, Post, Delete, Get, Put, Body, Param, UseGuards, Request, Query } from '@nestjs/common';
import { WatchlistService } from './watchlist.service';
import { JwtAuthGuard } from '../auth/jwt-auth.guard';

@Controller('watchlist')
export class WatchlistController {
  constructor(private watchlistService: WatchlistService) {}

  @Post()
  @UseGuards(JwtAuthGuard)
  async addToWatchlist(@Body() body: any, @Request() req: any) {
    return this.watchlistService.addToWatchlist(req.user.id, body.anime_id, body.status);
  }

  @Get()
  @UseGuards(JwtAuthGuard)
  async getWatchlist(@Request() req: any, @Query('status') status?: string) {
    return this.watchlistService.getWatchlist(req.user.id, status);
  }

  @Put(':id')
  @UseGuards(JwtAuthGuard)
  async updateStatus(@Param('id') id: string, @Body() body: any, @Request() req: any) {
    return this.watchlistService.updateStatus(req.user.id, id, body.status);
  }

  @Delete(':id')
  @UseGuards(JwtAuthGuard)
  async removeFromWatchlist(@Param('id') id: string, @Request() req: any) {
    return this.watchlistService.removeFromWatchlist(req.user.id, id);
  }
}
