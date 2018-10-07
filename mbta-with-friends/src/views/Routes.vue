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
            <p>Arriva Time: {{schedule[stop.api_id]}}</p>
          </md-card-content>
        </md-ripple>
      </md-card>
    </div>
  </div>
</template>

<script lang="ts">
import {Component, Vue, Watch} from 'vue-property-decorator';
import {State, Action} from 'vuex-class';
import {AxiosResponse} from 'axios';
import {Line} from '@/models';

@Component({})
export default class Routes extends Vue {
  @State('lines') lines: any;
  @State('stops') stops: any;
  @State('user') user: any;
  @State('api_key') api_key: any;
  @Action('getRoutes') getRoutes: any;

  public schedule: any = {};

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

  @Watch('lines', {immediate: true, deep: true})
  getScheduleForLines(lines: any, oldLines: any) {
    let routes = Object.keys(lines);

    if (routes.length > 0) {
      let stops = Object.values(lines)
        .map((s_lines: any) => s_lines.map((line: any) => line.stops).flat())
        .flat();

      let now = new Date();
      this.axios
        .get('https://api-v3.mbta.com/schedules', {
          params: {
            api_key: this.api_key,
            include: 'stop',
            'filter[min_time]': `${now.getHours()}:${now.getMinutes()}`,
            'filter[max_time]': `${now.getHours() + 1}:${now.getMinutes()}`,
            'filter[route_type]': 1,
            'filter[stop]': stops.map(stop => stop.api_id).join(','),
          },
        })
        .then((response: AxiosResponse<any>) => {
          let included = response.data.included;
          let ids: any = {};
          included.forEach((incl: any) => {
            let parentId: any = incl.relationships;
            if (parentId) {
              parentId = parentId.parent_station;
              if (parentId) {
                parentId = parentId.data.id;
              } else {
                parentId = null;
              }
            } else {
              parentId = null;
            }

            if (parentId) {
              ids[parentId] = parentId;
              ids[incl.id] = parentId;
            } else {
              ids[incl.id] = incl.id;
            }
          });

          let newSchedule: any = {};

          response.data.data.reverse().forEach((sched: any) => {
            newSchedule[ids[sched.relationships.stop.data.id]] =
              sched.attributes.arrival_time;
          });

          this.schedule = newSchedule;
        });
    }
  }
}
</script>
