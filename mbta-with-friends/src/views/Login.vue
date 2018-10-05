<template>
  <div class="login">
    <LoginUser/>
  </div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import LoginUser from '@/components/LoginUser.vue'; // @ is an alias to /src

@Component({
  components: {
    LoginUser,
  },
})
export default class Login extends Vue {}
</script>
