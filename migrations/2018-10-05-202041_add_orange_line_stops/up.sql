-- Your SQL goes here
INSERT INTO lines (name)
VALUES ('Orange');

INSERT INTO sublines (line_id, name, api_id)
VALUES (currval('lines_id_seq'), 'Orange', 'Orange');

INSERT INTO stops (name, api_id, longitude, latitude)
VALUES ('Forest Hills', 'place-forhl', -71.113686, 42.300523),
('Green Street', 'place-grnst', -71.107414, 42.310525),
('Stony Brook', 'place-sbml', -71.104248, 42.317062),
('Jackson Square', 'place-jaksn', -71.099592, 42.323132),
('Roxbury Crossing', 'place-rcmnl', -71.095451, 42.331397),
('Ruggles', 'place-rugg', -71.088961, 42.336377),
('Massachusetts Avenue', 'place-masta', -71.083423, 42.341512),
('Back Bay', 'place-bbsta', -71.075727, 42.34735),
('Tufts Medical Center', 'place-tumnl', -71.063917, 42.349662),
('Chinatown', 'place-chncl', -71.062752, 42.352547),
('Downtown Crossing', 'place-dwnxg', -71.060225, 42.355518),
('State Street', 'place-state', -71.057598, 42.358978),
('Haymarket', 'place-haecl', -71.05829, 42.363021),
('North Station', 'place-north', -71.06129, 42.365577),
('Community College', 'place-ccmnl', -71.069533, 42.373622),
('Sullivan Square', 'place-sull', -71.076994, 42.383975),
('Assembly', 'place-astao', -71.077257, 42.392811),
('Wellington', 'place-welln', -71.077082, 42.40237),
('Malden Center', 'place-mlmnl', -71.07411, 42.426632),
('Oak Grove', 'place-ogmnl', -71.071097, 42.43668);

CREATE TEMPORARY SEQUENCE tmp START 1;
INSERT INTO subline_stops (stop_id, subline_id, stop_sequence)
SELECT id, currval('sublines_id_seq'), nextval('tmp')
FROM stops;
