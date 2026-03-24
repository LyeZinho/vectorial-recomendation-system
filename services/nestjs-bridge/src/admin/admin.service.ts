import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { User } from '../users/entities/user.entity';
import axios from 'axios';

@Injectable()
export class AdminService {
  private readonly rustApiUrl = process.env.RUST_API_URL || 'http://localhost:3000';

  constructor(
    @InjectRepository(User)
    private usersRepo: Repository<User>,
  ) {}

  async getStats() {
    try {
      const totalUsers = await this.usersRepo.count();
      const adminUsers = await this.usersRepo.count({ where: { role: 'admin' } });

      const rustStats = await axios.get(`${this.rustApiUrl}/api/admin/stats`).catch(() => ({
        data: {
          total_anime: 0,
          total_recommendations: 0,
          avg_recommendation_score: 0,
        },
      }));

      return {
        users: {
          total: totalUsers,
          admins: adminUsers,
        },
        recommendations: {
          total: rustStats.data.total_recommendations || 0,
          avg_score: rustStats.data.avg_recommendation_score || 0,
        },
        anime: {
          total: rustStats.data.total_anime || 0,
        },
        system: {
          uptime: process.uptime(),
          environment: process.env.NODE_ENV || 'development',
        },
      };
    } catch (error: any) {
      throw new Error(`Failed to fetch stats: ${error.message}`);
    }
  }

  async getUsers(limit: number = 50, offset: number = 0) {
    try {
      const [users, total] = await this.usersRepo.findAndCount({
        skip: offset,
        take: limit,
        select: ['id', 'email', 'username', 'role', 'created_at'],
      });

      return {
        users,
        total,
        limit,
        offset,
      };
    } catch (error: any) {
      throw new Error(`Failed to fetch users: ${error.message}`);
    }
  }

  async updateUserRole(userId: string, role: 'user' | 'admin') {
    try {
      const user = await this.usersRepo.findOne({ where: { id: userId } });
      if (!user) {
        throw new Error('User not found');
      }

      user.role = role;
      await this.usersRepo.save(user);

      return {
        id: user.id,
        email: user.email,
        username: user.username,
        role: user.role,
      };
    } catch (error: any) {
      throw new Error(`Failed to update user role: ${error.message}`);
    }
  }

  async deleteUser(userId: string) {
    try {
      const user = await this.usersRepo.findOne({ where: { id: userId } });
      if (!user) {
        throw new Error('User not found');
      }

      await this.usersRepo.remove(user);
      return { message: 'User deleted successfully' };
    } catch (error: any) {
      throw new Error(`Failed to delete user: ${error.message}`);
    }
  }
}
