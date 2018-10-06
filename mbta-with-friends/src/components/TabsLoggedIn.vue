<template>
  <md-app md-waterfall md-mode="fixed">
    <md-app-toolbar class="md-primary">
      <div class="md-toolbar-row">
        <md-tabs class="md-primary" :md-active-tab="activeTab">
          <md-tab id="tab-home" md-label="Home" to="/"></md-tab>
          <md-tab id="tab-routes" md-label="Routes" to="/routes"></md-tab>
          <md-tab id="tab-add-friend" md-label="Add Friend" to="/add-friend"></md-tab>
        </md-tabs>
        <md-button id="tab-logout" v-on:click.prevent="logout" >Logout</md-button>
      </div>
    </md-app-toolbar>

    <md-app-content>
      <router-view/>
    </md-app-content>
  </md-app>
</template>

<script lang="ts">
import {Component, Vue, Prop} from 'vue-property-decorator';
import {State, Mutation} from 'vuex-class';
@Component
export default class TabsLoggedIn extends Vue {
  @State('user') user: any;
  @Mutation('set_user') setUser: any;
  @Mutation('set_users') setUsers: any;

  get activeTab() {
    let route = this.$route.path.slice(1);
    route = route == '' ? 'home' : route;
    return `tab-${route}`;
  }

  private logout() {
    this.axios
      .delete('/api/session')
      .then(() => {
        this.setUser(null);
        this.setUsers([]);

        (this as any).$snotify.success('Logged Out!');
        this.$router.push('/');
      })
      .catch(error => {
        if (error.response && error.response.data) {
          const data: any = error.response.data;
          (this as any).$snotify.error(data);
        }
      });
  }
}
</script>
