import { Controller, Get, Param, UseGuards, Request } from '@nestjs/common';
import { RecommendationsService } from './recommendations.service';
import { JwtAuthGuard } from '../auth/jwt-auth.guard';

@Controller('recommendations')
export class RecommendationsController {
  constructor(private recommendationsService: RecommendationsService) {}

  @Get(':id')
  @UseGuards(JwtAuthGuard)
  async getRecommendations(@Param('id') id: string, @Request() req: any) {
    const recommendations = await this.recommendationsService.getRecommendations(id);
    return {
      userId: id,
      recommendations: recommendations.recommendations || [],
      error: recommendations.error || null,
    };
  }
}
