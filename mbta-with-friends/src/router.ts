import Vue from 'vue';
import store from './stores';
import axios, {AxiosResponse} from 'axios';
import Router from 'vue-router';
import Home from './views/Home.vue';
import About from './views/About.vue';
import Login from './views/Login.vue';
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
      path: '/about',
      name: 'about',
      component: About,
    },
    {
      path: '/login',
      name: 'login',
      component: Login,
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
