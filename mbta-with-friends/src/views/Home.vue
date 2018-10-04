<template>
  <div class="home">
    <HelloWorld msg="Welcome to Your Vue.js + TypeScript App"/>
    <LoginUser/>
  </div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import HelloWorld from '@/components/HelloWorld.vue'; // @ is an alias to /src
import LoginUser from '@/components/LoginUser.vue'; // @ is an alias to /src

@Component({
  components: {
    HelloWorld,
    LoginUser,
  },
})
export default class Home extends Vue {}
</script>
