import { Controller, Get, Put, Delete, UseGuards, Request, Body, Param, ForbiddenException } from '@nestjs/common';
import { UsersService } from './users.service';
import { JwtAuthGuard } from '../auth/jwt-auth.guard';
import { UpdateUserDto } from './dtos/update-user.dto';

@Controller('users')
export class UsersController {
  constructor(private usersService: UsersService) {}

  @Get('me')
  @UseGuards(JwtAuthGuard)
  async getMe(@Request() req) {
    return {
      id: req.user.id,
      email: req.user.email,
      username: req.user.username,
      profile: req.user.profile,
      preferences: req.user.preferences,
    };
  }

  @Get(':id')
  @UseGuards(JwtAuthGuard)
  async getUser(@Param('id') id: string) {
    const user = await this.usersService.findById(id);
    return {
      id: user.id,
      username: user.username,
      profile: user.profile,
    };
  }

  @Put(':id')
  @UseGuards(JwtAuthGuard)
  async updateUser(@Param('id') id: string, @Body() updateUserDto: UpdateUserDto, @Request() req) {
    if (req.user.id !== id && req.user.role !== 'admin') {
      throw new ForbiddenException('Cannot update other users');
    }

    const updateData: any = {};
    if (updateUserDto.username) updateData.username = updateUserDto.username;
    if (updateUserDto.bio || updateUserDto.favorite_genre) {
      updateData.profile = {
        ...req.user.profile,
        ...(updateUserDto.bio && { bio: updateUserDto.bio }),
        ...(updateUserDto.favorite_genre && { favorite_genre: updateUserDto.favorite_genre }),
      };
    }
    if (updateUserDto.theme) {
      updateData.preferences = {
        ...req.user.preferences,
        theme: updateUserDto.theme,
      };
    }

    const updated = await this.usersService.update(id, updateData);
    return {
      id: updated.id,
      email: updated.email,
      username: updated.username,
      profile: updated.profile,
      preferences: updated.preferences,
    };
  }

  @Delete(':id')
  @UseGuards(JwtAuthGuard)
  async deleteUser(@Param('id') id: string, @Request() req) {
    if (req.user.id !== id && req.user.role !== 'admin') {
      throw new ForbiddenException('Cannot delete other users');
    }

    await this.usersService.update(id, { is_deleted: true });
    return { message: 'User deleted successfully' };
  }
}
