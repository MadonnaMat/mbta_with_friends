export interface ILine {
  id: number;
  stop_sequence: number;
  name: string;
  subline_name: string;
  api_id: string;
}

export interface IRoute {
  id: number;
  name: string;
  api_id: string;
  longitude: number;
  latitude: number;
  lines: ILine[];
}

export class Line {
  public id: number;
  public name: string;
  public subline_name: string;
  public api_id: string;
  public stops: Stop[];

  constructor(line: ILine) {
    this.id = line.id;
    this.name = line.name;
    this.subline_name = line.subline_name;
    this.api_id = line.api_id;
    this.stops = [];
  }
}

export class Stop {
  public id: number;
  public name: string;
  public api_id: string;
  public longitude: number;
  public latitude: number;
  public lines: {line: Line; stop_sequence: number}[];

  constructor(route: IRoute, state: any) {
    this.id = route.id;
    this.name = route.name;
    this.api_id = route.api_id;
    this.longitude = route.longitude;
    this.latitude = route.latitude;

    let line_ids = route.lines.map(line => line.id);
    let lines: Line[] = ((Object.values(state.lines) || []) as Line[]).filter(
      line => line_ids.includes(line.id),
    );

    if (lines.length != line_ids.length) {
      const done_line_ids = lines.map(line => line.id);
      line_ids = line_ids.filter(line_id => !done_line_ids.includes(line_id));
      route.lines.forEach(line => {
        if (line_ids.includes(line.id)) {
          let newLine = new Line(line);
          lines.push(newLine);
          state.lines[line.name] = state.lines[line.name] || [];
          state.lines[line.name].push(newLine);
        }
      });
    }

    this.lines = [];
    let seqs: any = {};

    route.lines.forEach(line => {
      seqs[line.id] = line.stop_sequence;
    });

    lines.forEach(line => {
      this.lines.push({
        line: line,
        stop_sequence: seqs[line.id],
      });

      line.stops.push(this);
    });
  }
}
