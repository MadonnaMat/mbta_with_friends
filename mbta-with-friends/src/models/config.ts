import {User} from '@/models';

export interface Config {
  user: User | null;
  api_key: string;
}
