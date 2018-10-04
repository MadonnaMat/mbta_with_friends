import Vue from 'vue';
import Vuex from 'vuex';
import users from './users';

Vue.use(Vuex);

let store = new Vuex.Store({
  ...(users as any),
});

export default store;
