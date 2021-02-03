CREATE TABLE users
(
  id SERIAL PRIMARY KEY,
  email varchar(255) NOT NULL,
  first_name varchar(255) NOT NULL,
  last_name varchar(255) NOT NULL,
  "password" varchar(255) NOT NULL
);

INSERT INTO 
    users(email, first_name, last_name, "password")
VALUES
    ('test+1@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+2@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+3@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+4@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+5@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+6@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+7@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+8@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+9@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+10@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+11@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+12@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+13@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+14@test.com', 'John', 'Doe', 'some-hashed-password'),
    ('test+15@test.com', 'John', 'Doe', 'some-hashed-password');
