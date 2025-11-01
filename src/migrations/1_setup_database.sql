CREATE TABLE IF NOT EXISTS turn (
  id BINARY(16) PRIMARY KEY,
  ipv4 INTEGER UNSIGNED,
  ipv6 BINARY(16),
  time_to_first_token INTEGER UNSIGNED,
  time_to_last_token INTEGER UNSIGNED,
  model CHAR,
  guess CHAR,
  sentiment INTEGER,
  time TIMESTAMP
);