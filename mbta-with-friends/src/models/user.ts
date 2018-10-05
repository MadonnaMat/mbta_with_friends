export interface IUser {
  id: number;
  username: string;
}

export interface NUser {
  username: string;
  password: string;
}

export class User implements IUser {
  public id: number;
  public username: string;

  constructor(user: IUser) {
    this.id = user.id;
    this.username = user.username;
  }
}
