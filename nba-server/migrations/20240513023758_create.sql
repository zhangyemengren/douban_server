-- database
# create database nba-data;
ALTER DATABASE `nba-data` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
-- tables
CREATE TABLE team
(
    id           int primary key auto_increment,
    name         varchar(255) not null,
    full_name    varchar(255) not null,
    city         varchar(255) not null,
    abbreviation varchar(255) not null,
    conference   varchar(255) not null,
    division     varchar(255) not null
);

