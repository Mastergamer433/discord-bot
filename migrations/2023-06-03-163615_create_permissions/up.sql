-- Your SQL goes here
CREATE TABLE permissions(
  `id` INTEGER AUTO_INCREMENT NOT NULL,
  `guild_id` VARCHAR(191) NOT NULL,
  `user_id` VARCHAR(191) NOT NULL,
  `permission_string` VARCHAR(2048) NOT NULL,
  primary key(`id`)
);
