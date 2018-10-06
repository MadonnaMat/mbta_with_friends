<template>
  <div class="routes">
    <div v-for="(l_stops, line) in lines_and_stops">
      <h1>{{line}}</h1>
      <md-card md-with-hover v-for="stop in l_stops">
        <md-ripple>
          <md-card-header>
            <div class="md-title">
              {{ stop.name }}
            </div>
          </md-card-header>

          <md-card-content>
            <p>Id: {{ stop.id }}</p>
            <p>Api Id: {{stop.api_id}}</p>
          </md-card-content>
        </md-ripple>
      </md-card>
    </div>
  </div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import {State, Action} from 'vuex-class';

@Component({})
export default class Routes extends Vue {
  @State('lines') lines: any;
  @State('stops') stops: any;
  @State('user') user: any;
  @Action('getRoutes') getRoutes: any;

  get lines_and_stops() {
    let keys = Object.keys(this.lines);

    let obj: any = {};

    keys.forEach(key => {
      obj[key] = this.lines[key]
        .map((line: any) => {
          return line.stops.map((stop: any) => {
            return {
              id: stop.id,
              name: stop.name,
              api_id: stop.api_id,
            };
          });
        })
        .flat();
    });

    return obj;
  }

  private mounted(): void {
    if (this.user && this.stops.length == 0) {
      this.getRoutes();
    }
  }
}
</script>
