CREATE TABLE IF NOT EXISTS turn (
  id BINARY(16) PRIMARY KEY,
  ipv4 BINARY(4),
  ipv6 BINARY(16),
  time_to_first_token INTEGER UNSIGNED,
  time_to_last_token INTEGER UNSIGNED,
  total_length INTEGER UNSIGNED,
  model CHAR(1),
  guess CHAR(1),
  sentiment INTEGER,
  time TIMESTAMP
);