import Vapi from 'vuex-rest-api';
import {Stop} from '@/models';

const routes = new Vapi({
  state: {
    stops: [],
    lines: {},
  },
});

export default routes
  .get({
    action: 'getRoutes',
    property: 'routes',
    path: '/api/routes',
    onSuccess: (state: any, payload: any) => {
      state.stops = payload.data.map((route: any) => new Stop(route, state));
    },
  })
  .getStore();
