<template>
  <div>
    <div v-if="user">
      {{ user.username }}
    </div>
    <form novalidate class="md-layout" v-on:submit.prevent="onSubmit">
      <md-card class="md-layout-item md-size-50 md-small-size-100">
        <md-card-header>
          <div class="md-title">
            Log In
          </div>
        </md-card-header>

        <md-card-content>
          <div class="md-layout md-gutter">
            <div class="md-layout-item md-small-size-100">
              <md-field>
                <label for="username">User Name</label>
                <md-input name="username" id="username" v-model="form.username" :disabled="sending" />
              </md-field>
            </div>

            <div class="md-layout-item md-small-size-100">
              <md-field>
                <label for="password">Password</label>
                <md-input name="password" id="password" v-model="form.password" :disabled="sending" />
              </md-field>
            </div>

            <md-progress-bar md-mode="indeterminate" v-if="sending" />

            <md-card-actions>
              <md-button type="submit" class="md-primary" :disabled="sending">
                Log In
              </md-button>
            </md-card-actions>

          </div>
        </md-card-content>
      </md-card>
    </form>
  </div>
</template>

<script lang="ts">
import Component from 'vue-class-component';
import {Prop, Vue} from 'vue-property-decorator';
import {NUser, IUser, User} from '@/models/';
import {AxiosResponse} from 'axios';
import {State, Action} from 'vuex-class';

@Component
export default class LoginUser extends Vue {
  @State('user') user: any;

  private sending: boolean = false;
  private form: NUser = {username: '', password: ''};

  private onSubmit(): void {
    this.axios
      .post<IUser>('/api/session', this.form)
      .then((response: AxiosResponse<IUser>) => {
        let newUser: User = response.data;
        console.log(response);

        this.user = newUser;

        (this as any).$snotify.success('Logged In!');
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

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
