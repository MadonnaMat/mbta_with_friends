import Vue from 'vue';
import Vuex from 'vuex';
import users from './users';
import routes from './routes';
import {Config, User} from '@/models';
import _ from 'lodash';

Vue.use(Vuex);

let defaults = {
  state: {
    user: null,
    hasConfig: false,
  },
  mutations: {
    set_config(state: any, config: Config) {
      state.hasConfig = true;
      state.user = config.user;
    },
    set_user(state: any, user: User) {
      state.user = user;
    },
    set_users(state: any, users: User[]) {
      state.users = users;
    },
  },
};
let users_obj = users as any;
let routes_obj = routes as any;
let store_obj = _.merge(defaults, users_obj, routes_obj);

let store = new Vuex.Store(store_obj);

export default store;
