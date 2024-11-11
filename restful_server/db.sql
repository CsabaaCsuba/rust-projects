drop database if exists gambling_game;

create database gambling_game character set "utf8" collate "utf8_hungarian_ci";
use gambling_game;
create table gamblers (
    id int not null auto_increment,
    username varchar(255) not null, 
    email varchar(255) not null, 
    password varchar(255) not null,
    constraint primary key (id)
);

create table matches (
    id int not null auto_increment,
    user1_id int not null,
    user2_id int not null,

    gamestate char, 
    -- 0 unfinished, 1, player1 won, 2 player2 won, 3 draw

    -- gamemap bigint, 
--TODO: write an algorithm that maps a 10x10 map with the standard amount of ships (1x4, 2x3, 3x2, 4x1) 
--      and their position (horizontal or vertical) to a 64-bit integer


    --each bit represents a map tile on a 10x10 map. there are 28 remaining bits. The upper 14 bits are unused in both cases
    --mapstate1 represents the upper 5 rows, and the mapstate2 the lower 5 rows
    --2^50 = 1,125,899,906,842,624 //this is the highest number possible
    user1_mapstate1 bigint, 
    user1_mapstate2 bigint,

    user1_hitmap1 bigint,
    user1_hitmap2 bigint,

    /*
    thought process on a 4x4 grid (if there were only 8-bit CPUs)
     ---------------
    | 1 | 2 | 3 | 4 |
    | 5 | 6 | 7 | 8 |
     -> then the other storage's bits are loaded
    | 1 | 2 | 3 | 4 |
    | 5 | 6 | 7 | 8 |
     ---------------

    
    */
    

    user2_mapstate1 bigint,
    user2_mapstate2 bigint,

    user2_hitmap1 bigint,
    user2_hitmap2 bigint,

    constraint primary key (id),
    constraint foreign key (user1_id) references gambler(id),
    constraint foreign key (user2_id) references gambler(id)
);


-- create table maptile (
--     id int not null auto_increment,
--     match_id int not null,

--     tilestate char
--     -- 0 unexplored, 1 player

--     constraint primary key (id),
--     constraint foreign key (user1_id) references gambler(id),
-- )