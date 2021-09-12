-- Your SQL goes here
create table if not exists users
(
    id         int         not null primary key auto_increment,
    name       varchar(10) not null default '',
    is_deleted tinyint     not null default 0
) ENGINE = InnoDB
  DEFAULT charset = utf8mb4;