<template>
  <div>
    <form novalidate class="md-layout">
      <md-card class="md-layout-item md-size-50 md-small-size-100">
        <md-card-header>
          <div class="md-title">
            Add A Friend
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

            <md-progress-bar md-mode="indeterminate" v-if="sending" />

            <md-card-actions>
              <md-button type="submit" name="login" class="md-primary" :disabled="sending" v-on:click.prevent="onSubmit">
                Add Friend
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
import {NFriend, IFriend, Friend} from '@/models/';
import {AxiosResponse} from 'axios';
import {State, Mutation} from 'vuex-class';

@Component
export default class LoginUser extends Vue {
  @State('user') user: any;
  @Mutation('set_users') setUsers: any;

  private sending: boolean = false;
  private form: NFriend = {username: ''};

  private onSubmit(): void {
    this.sending = true;
    this.axios
      .post<IFriend[]>('/api/friends', this.form)
      .then((response: AxiosResponse<IFriend[]>) => {
        let friends: Friend[] = response.data.map(friend => new Friend(friend));
        this.setUsers(friends);

        this.$router.push('/', () => {
          (this as any).$snotify.success('Friend Added!');
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

  private mounted(): void {
    if (!this.user) {
      this.$router.push('/');
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
