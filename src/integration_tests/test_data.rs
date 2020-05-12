pub static CREATE_SQL_STMT: &'static str = "DROP DATABASE IF EXISTS `iot_project_db`;
CREATE SCHEMA `iot_project_db` DEFAULT CHARACTER SET latin1;
USE iot_project_db;
create table `location` (`name` varchar(128) NOT NULL ,
`id` integer NOT NULL UNIQUE AUTO_INCREMENT ,
 PRIMARY KEY( `id`));
create table `rfid_tracker` (`id` varchar(64) NOT NULL UNIQUE ,
`location` integer ,
FOREIGN KEY (`location`)
REFERENCES location(`id`),
 PRIMARY KEY( `id`));
create table `rfid_receiver` (`id` varchar(64) NOT NULL UNIQUE ,
`location` integer NOT NULL ,
FOREIGN KEY (`location`)
REFERENCES location(`id`),
 PRIMARY KEY( `id`));
create table `interest` (`name` varchar(128) NOT NULL UNIQUE ,
`id` integer NOT NULL UNIQUE AUTO_INCREMENT ,
 PRIMARY KEY( `id`));
create table `tracker_interest` (`interest` integer NOT NULL ,
`tracker` varchar(64) NOT NULL ,
`weight` float NOT NULL ,
FOREIGN KEY (`interest`)
REFERENCES interest(`id`),
FOREIGN KEY (`tracker`)
REFERENCES rfid_tracker(`id`),
 PRIMARY KEY( `interest`, `tracker`));
create table `display` (`id` integer NOT NULL UNIQUE AUTO_INCREMENT ,
`location` integer NOT NULL ,
FOREIGN KEY (`location`)
REFERENCES location(`id`),
 PRIMARY KEY( `id`, `location`));
create table `advertisement_video` (`id` integer NOT NULL UNIQUE AUTO_INCREMENT ,
`interest` integer NOT NULL ,
`length_sec` integer NOT NULL ,
`url` varchar(255) NOT NULL ,
FOREIGN KEY (`interest`)
REFERENCES interest(`id`),
 PRIMARY KEY( `id`));
create table `agency` (`orgnr` varchar(128) NOT NULL UNIQUE ,
`name` varchar(128) NOT NULL UNIQUE ,
 PRIMARY KEY( `orgnr`));
create table `users` (`username` varchar(128) NOT NULL UNIQUE ,
`email` varchar(128) NOT NULL UNIQUE ,
`agency` varchar(128) NOT NULL ,
`pass_hash` varchar(128) NOT NULL ,
FOREIGN KEY (`agency`)
REFERENCES agency(`orgnr`),
 PRIMARY KEY( `username`));
create table `orders` (`id` varchar(40) NOT NULL UNIQUE ,
`credits` integer NOT NULL ,
`user` varchar(128) NOT NULL ,
FOREIGN KEY (`user`)
REFERENCES users(`email`),
 PRIMARY KEY( `id`));
create table `played_video` (`id` integer NOT NULL UNIQUE AUTO_INCREMENT ,
`video` integer NOT NULL ,
`time_epoch` integer NOT NULL ,
`order` varchar(40) NOT NULL ,
FOREIGN KEY (`video`)
REFERENCES advertisement_video(`id`),
FOREIGN KEY (`order`)
REFERENCES orders(`id`),
 PRIMARY KEY( `id`));
create table `advertisement_order` (`video` integer NOT NULL ,
`orders` varchar(40) NOT NULL ,
`start_time_epoch` integer NOT NULL ,
`end_time_epoch` integer NOT NULL ,
FOREIGN KEY (`video`)
REFERENCES advertisement_video(`id`),
FOREIGN KEY (`orders`)
REFERENCES orders(`id`),
 PRIMARY KEY( `video`, `orders`));
CREATE TABLE refresh_token (`refresh_token` BLOB, INDEX(`refresh_token`(10)), id varchar(40) NOT NULL ,
  PRIMARY KEY(id));";