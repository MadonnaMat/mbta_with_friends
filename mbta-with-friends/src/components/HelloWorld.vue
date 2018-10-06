<template>
  <div>
    <div>
      <md-card md-with-hover v-for="user in users">
        <md-ripple>
          <md-card-header>
            <div class="md-title">
              {{ user.username }}
            </div>
          </md-card-header>

          <md-card-content>
            <p>Id: {{ user.id }}</p>
            <p>Is Friend: {{user.is_friend}}</p>
          </md-card-content>
        </md-ripple>
      </md-card>
    </div>
  </div>
</template>

<script lang="ts">
import Component from 'vue-class-component';
import {Prop, Vue} from 'vue-property-decorator';
import {NUser, User} from '@/models/';
import {AxiosResponse} from 'axios';
import {State, Action} from 'vuex-class';

@Component
export default class HelloWorld extends Vue {
  @Prop() private msg!: string;
  @State('users') users: any;
  @State('user') user: any;
  @Action('listUsers') listUsers: any;

  private sending: boolean = false;
  private form: NUser = {username: '', password: ''};

  private mounted(): void {
    if (this.user && this.users.length == 0) {
      this.listUsers();
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
