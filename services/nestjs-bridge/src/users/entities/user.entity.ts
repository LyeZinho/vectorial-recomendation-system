import { Entity, PrimaryGeneratedColumn, Column, CreateDateColumn, UpdateDateColumn } from 'typeorm';

@Entity('users')
export class User {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ unique: true })
  email: string;

  @Column({ unique: true })
  username: string;

  @Column()
  password_hash: string;

  @Column({ default: 'user' })
  role: 'user' | 'admin';

  @Column({ type: 'json', nullable: true })
  profile: {
    avatar_url?: string;
    bio?: string;
    favorite_genre?: string;
  };

  @Column({ type: 'json', nullable: true })
  preferences: {
    theme: 'light' | 'dark';
    language: string;
    notifications_enabled: boolean;
  };

  @Column({ default: false })
  is_deleted: boolean;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;
}
