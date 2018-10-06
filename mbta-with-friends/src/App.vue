<template>
  <div v-if="user"  class="page-container">
    <TabsLoggedIn/>
    <vue-snotify/>
  </div>
  <div v-else class="page-container">
    <md-app md-waterfall md-mode="fixed">
      <md-app-toolbar class="md-primary">
      <div class="md-toolbar-row">
        <md-tabs class="md-primary" :md-active-tab="activeTab">
          <md-tab id="tab-home" md-label="Home" to="/"></md-tab>
          <md-tab id="tab-login" md-label="Login" to="/login"></md-tab>
        </md-tabs>
      </div>
      </md-app-toolbar>

      <md-app-content>
        <router-view/>
      </md-app-content>
    </md-app>
    <vue-snotify/>
  </div>
</template>

<script lang="ts">
import Component from 'vue-class-component';
import {Vue} from 'vue-property-decorator';
import {State, Mutation} from 'vuex-class';
import TabsLoggedIn from '@/components/TabsLoggedIn.vue'; // @ is an alias to /src

@Component({
  components: {
    TabsLoggedIn,
  },
})
export default class App extends Vue {
  @State('user') user: any;
  @Mutation('set_user') setUser: any;
  @Mutation('set_users') setUsers: any;

  get loggedIn() {
    return !!this.user;
  }

  get notLoggedIn() {
    return !!!this.user;
  }

  get activeTab() {
    let route = this.$route.path.slice(1);
    route = route == '' ? 'home' : route;
    return `tab-${route}`;
  }
}
</script>

<style lang="scss">
@import '~vue-snotify/styles/material';
</style>
