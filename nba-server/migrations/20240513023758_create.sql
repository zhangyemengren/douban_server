-- database
create database nba DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
-- tables
create table team (
  id int primary key auto_increment,
  name varchar(255) not null,
  city varchar(255) not null,
  abbreviation varchar(255) not null
);

