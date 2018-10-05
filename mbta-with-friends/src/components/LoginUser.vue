<template>
  <div>
    <div v-if="user">
      {{ user.username }}
    </div>
    <form novalidate class="md-layout">
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
                <md-input name="password" id="password" v-model="form.password" type="password" :disabled="sending" />
              </md-field>
            </div>

            <md-progress-bar md-mode="indeterminate" v-if="sending" />

            <md-card-actions>
              <md-button type="submit" name="create" class="md-primary" :disabled="sending" v-on:click.prevent="onSubmit('new')">
                New User
              </md-button>
              <md-button type="submit" name="login" class="md-primary" :disabled="sending" v-on:click.prevent="onSubmit('login')">
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
import {State, Mutation} from 'vuex-class';

@Component
export default class LoginUser extends Vue {
  @State('user') user: any;
  @Mutation('set_user') setUser: any;
  @Mutation('set_users') setUsers: any;

  private sending: boolean = false;
  private form: NUser = {username: '', password: ''};

  private onSubmit(event: string): void {
    if (event == 'new') {
      this.onSubmitCreate();
    } else {
      this.onSubmitLogin();
    }
  }

  private mounted(): void {
    if (this.user) {
      this.$router.push('/');
    }
  }

  private onSubmitLogin(): void {
    this.sending = true;
    this.axios
      .post<IUser>('/api/session', this.form)
      .then((response: AxiosResponse<IUser>) => {
        let newUser: User = response.data;
        this.setUser(newUser);
        this.setUsers([]);

        this.$router.push('/', () => {
          (this as any).$snotify.success('Logged In!');
        });
      })
      .catch(error => {
        if (error.response && error.response.data) {
          const data: any = error.response.data;
          (this as any).$snotify.error(data);
        }
      })
      .finally(() => {
        this.sending = false;
      });
  }

  private onSubmitCreate(): void {
    this.sending = true;
    this.axios
      .post<IUser>('/api/users', this.form)
      .then((response: AxiosResponse<IUser>) => {
        let newUser: User = response.data;
        this.setUser(newUser);
        this.setUsers([]);

        this.$router.push('/', () => {
          (this as any).$snotify.success('User Created!');
        });
      })
      .catch(error => {
        if (error.response && error.response.data) {
          const data: any = error.response.data;
          (this as any).$snotify.error(data);
        }
      })
      .finally(() => {
        this.sending = false;
      });
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
