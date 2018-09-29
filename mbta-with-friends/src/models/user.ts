export interface IUser {
  id: number;
  username: string;
  password: string;
}

export interface NUser {
  username: string;
  password: string;
}

export class User implements IUser {
  public id: number;
  public username: string;
  public password: string;

  constructor(user: IUser) {
    this.id = user.id;
    this.username = user.username;
    this.password = user.password;
  }
}
