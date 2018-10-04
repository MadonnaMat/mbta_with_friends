import Vapi from 'vuex-rest-api';

const users = new Vapi({
  state: {
    user: null,
    users: [],
  },
});

export default users
  .get({
    action: 'listUsers',
    property: 'users',
    path: '/api/users',
  })
  .getStore();
