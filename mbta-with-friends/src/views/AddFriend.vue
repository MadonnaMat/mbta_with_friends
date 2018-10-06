<template>
  <div class="add-friend">
    <AddAFriend/>
  </div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import AddAFriend from '@/components/AddAFriend.vue'; // @ is an alias to /src

@Component({
  components: {
    AddAFriend,
  },
})
export default class AddFriend extends Vue {}
</script>
