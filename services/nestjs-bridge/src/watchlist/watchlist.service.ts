import { Injectable, NotFoundException, BadRequestException } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Watchlist } from './watchlist.entity';

@Injectable()
export class WatchlistService {
  constructor(
    @InjectRepository(Watchlist)
    private watchlistRepository: Repository<Watchlist>,
  ) {}

  async addToWatchlist(userId: string, animeId: number, status: string = 'watching') {
    const existing = await this.watchlistRepository.findOne({
      where: { user_id: userId, anime_id: animeId },
    });

    if (existing) {
      throw new BadRequestException('Anime already in watchlist');
    }

    const watchlistItem = this.watchlistRepository.create({
      user_id: userId,
      anime_id: animeId,
      status: status as 'watching' | 'completed' | 'dropped' | 'planned',
    });

    return this.watchlistRepository.save(watchlistItem);
  }

  async removeFromWatchlist(userId: string, id: string) {
    const item = await this.watchlistRepository.findOne({
      where: { id, user_id: userId },
    });

    if (!item) {
      throw new NotFoundException('Watchlist item not found');
    }

    await this.watchlistRepository.remove(item);
    return { message: 'Removed from watchlist' };
  }

  async getWatchlist(userId: string, status?: string) {
    const query = this.watchlistRepository.createQueryBuilder('watchlist')
      .where('watchlist.user_id = :userId', { userId });

    if (status) {
      query.andWhere('watchlist.status = :status', { status });
    }

    return query.orderBy('watchlist.added_at', 'DESC').getMany();
  }

  async updateStatus(userId: string, id: string, status: string) {
    const item = await this.watchlistRepository.findOne({
      where: { id, user_id: userId },
    });

    if (!item) {
      throw new NotFoundException('Watchlist item not found');
    }

    item.status = status as 'watching' | 'completed' | 'dropped' | 'planned';
    return this.watchlistRepository.save(item);
  }
}
