-- database
# create database nba;
ALTER DATABASE nba CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
-- tables
CREATE TABLE team
(
    id           int primary key auto_increment,
    name         varchar(255) not null,
    home_court         varchar(255) not null,
    abbreviation varchar(255) not null,
    area       varchar(255) not null,
    `partition`  varchar(255) not null
);

