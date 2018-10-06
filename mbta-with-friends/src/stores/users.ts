import Vapi from 'vuex-rest-api';
import {Friend} from '@/models';

const users = new Vapi({
  state: {
    users: [],
  },
});

export default users
  .get({
    action: 'listUsers',
    property: 'users',
    path: '/api/users',
    onSuccess: (state: any, payload: any) => {
      state.users = payload.data.map((friend: any) => new Friend(friend));
    },
  })
  .getStore();
