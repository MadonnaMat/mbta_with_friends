export interface IFriend {
  id: number;
  username: string;
  is_friend: boolean;
}

export interface NFriend {
  username: string;
}

export class Friend implements IFriend {
  public id: number;
  public username: string;
  public is_friend: boolean;

  constructor(friend: IFriend) {
    this.id = friend.id;
    this.username = friend.username;
    this.is_friend = friend.is_friend;
  }
}
