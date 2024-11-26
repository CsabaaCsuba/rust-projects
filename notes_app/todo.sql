create database tasks character set utf8;
use tasks;
create table task (
 id int not null auto_increment primary key,
 title char(255) not null,
 description text, 
 is_completed boolean default false
);
