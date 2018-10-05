<template>
  <div class="page-container">
    <md-app md-waterfall md-mode="fixed">
      <md-app-toolbar class="md-primary">
        <div class="md-toolbar-row">
          <md-tabs class="md-primary">
            <md-tab id="tab-home" md-label="Home" to="/"></md-tab>
            <md-tab id="tab-about" md-label="About" to="/about"></md-tab>
            <md-tab v-if="user == null" id="tab-login" md-label="Login" to="/login"></md-tab>
          </md-tabs>
          <md-button v-if="user" id="tab-logout" v-on:click.prevent="logout" >Logout</md-button>
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

@Component
export default class App extends Vue {
  @State('user') user: any;
  @Mutation('set_user') setUser: any;
  @Mutation('set_users') setUsers: any;

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

<style lang="scss">
@import '~vue-snotify/styles/material';
</style>
