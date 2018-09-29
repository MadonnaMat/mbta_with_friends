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
            <p>Password: {{ user.password }}</p>
          </md-card-content>
        </md-ripple>
      </md-card>
    </div>

    <form novalidate class="md-layout" v-on:submit.prevent="onSubmit">
      <md-card class="md-layout-item md-size-50 md-small-size-100">
        <md-card-header>
          <div class="md-title">
            New User
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
                Create User
              </md-button>
            </md-card-actions>

          </div>
        </md-card-content>
      </md-card>
    </form>
  </div>
</template>

<script lang="ts">
import {Component, Prop, Vue} from 'vue-property-decorator';
import {NUser, IUser, User} from '@/models/';
import {AxiosResponse} from 'axios';

@Component
export default class HelloWorld extends Vue {
  @Prop() private msg!: string;
  private users: User[] = [];
  private sending: boolean = false;
  private form: NUser = {username: '', password: ''};

  private mounted(): void {
    this.getUsers();
  }

  private getUsers(): void {
    this.axios
      .get<IUser[]>('/api/users')
      .then((response: AxiosResponse<IUser[]>) => {
        this.users = response.data.map(datum => new User(datum));
      });
  }

  private onSubmit(): void {
    this.axios
      .post<IUser>('/api/users', this.form)
      .then((response: AxiosResponse<IUser>) => {
        this.getUsers();
        (this as any).$snotify.success('User Created!');
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
