import { Injectable, ConflictException, NotFoundException } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { User } from './entities/user.entity';
import * as bcrypt from 'bcrypt';

@Injectable()
export class UsersService {
  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
  ) {}

  async create(email: string, username: string, password: string) {
    const existing = await this.usersRepository.findOne({
      where: [{ email }, { username }],
    });

    if (existing) {
      throw new ConflictException('Email or username already exists');
    }

    const password_hash = await bcrypt.hash(password, 10);

    const user = this.usersRepository.create({
      email,
      username,
      password_hash,
      role: 'user',
      preferences: {
        theme: 'dark',
        language: 'en',
        notifications_enabled: true,
      },
    });

    return this.usersRepository.save(user);
  }

  async findById(id: string) {
    const user = await this.usersRepository.findOne({ where: { id } });
    if (!user) {
      throw new NotFoundException('User not found');
    }
    return user;
  }

  async findByEmail(email: string) {
    return this.usersRepository.findOne({ where: { email } });
  }

  async update(id: string, updateData: Partial<User>) {
    await this.findById(id);
    await this.usersRepository.update(id, updateData);
    return this.findById(id);
  }

  async verifyPassword(user: User, password: string): Promise<boolean> {
    return bcrypt.compare(password, user.password_hash);
  }
}
