import Vue from 'vue';
import axios from 'axios';
import VueAxios from 'vue-axios';
import VueMaterial from 'vue-material';
import Snotify from 'vue-snotify';
import 'vue-material/dist/vue-material.min.css';
import 'vue-material/dist/theme/default.css';

import App from './App.vue';
import router from './router';
import store from './stores';
import './registerServiceWorker';

Vue.use(VueMaterial);
Vue.use(Snotify);
Vue.use(VueAxios, axios);
Vue.config.productionTip = false;

new Vue({
  router,
  store,
  render: h => h(App),
}).$mount('#app');
