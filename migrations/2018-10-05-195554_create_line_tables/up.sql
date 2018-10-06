-- Your SQL goes here
CREATE TABLE lines (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE sublines (
  id SERIAL PRIMARY KEY,
  line_id INTEGER REFERENCES lines,
  name VARCHAR NOT NULL,
  api_id VARCHAR NOT NULL
);

CREATE TABLE stops (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  api_id VARCHAR NOT NULL,
  longitude FLOAT NOT NULL,
  latitude FLOAT NOT NULL
);

CREATE TABLE subline_stops (
  id SERIAL PRIMARY KEY,
  subline_id INTEGER REFERENCES sublines NOT NULL,
  stop_id INTEGER REFERENCES stops NOT NULL,
  stop_sequence INTEGER NOT NULL
);
