import Vue from 'vue';
import store from './stores';
import axios, {AxiosResponse} from 'axios';
import Router from 'vue-router';
import Home from './views/Home.vue';
import Login from './views/Login.vue';
import AddFriend from './views/AddFriend.vue';
import Routes from './views/Routes.vue';
import {Config} from './models';

Vue.use(Router);

let router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    {
      path: '/login',
      name: 'login',
      component: Login,
    },
    {
      path: '/add-friend',
      name: 'add-friend',
      component: AddFriend,
    },
    {
      path: '/routes',
      name: 'routes',
      component: Routes,
    },
  ],
});

router.beforeEach((to, from, next) => {
  if ((store as any).state.hasConfig) {
    next();
  } else {
    axios.get('/api/config').then((response: AxiosResponse<Config>) => {
      store.commit('set_config', response.data);
      next();
    });
  }
});

export default router;
