import { Entity, PrimaryGeneratedColumn, Column, CreateDateColumn, ManyToOne, JoinColumn, Unique } from 'typeorm';
import { User } from '../users/entities/user.entity';

@Entity('watchlist')
@Unique(['user_id', 'anime_id'])
export class Watchlist {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column()
  user_id: string;

  @ManyToOne(() => User, { onDelete: 'CASCADE' })
  @JoinColumn({ name: 'user_id' })
  user: User;

  @Column()
  anime_id: number;

  @Column({ type: 'varchar', default: 'watching' })
  status: 'watching' | 'completed' | 'dropped' | 'planned';

  @CreateDateColumn()
  added_at: Date;
}
