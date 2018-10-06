-- Your SQL goes here
CREATE TABLE friends ( 
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users NOT NULL,
  friend_id INTEGER REFERENCES users NOT NULL
);

