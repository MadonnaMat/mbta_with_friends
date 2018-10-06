-- Your SQL goes here
ALTER TABLE friends
  ADD UNIQUE (user_id, friend_id);
