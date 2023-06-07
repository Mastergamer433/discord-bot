-- Your SQL goes here
CREATE TABLE options(
  `id` INTEGER AUTO_INCREMENT NOT NULL,
  `guild_id` VARCHAR(191) NOT NULL,
  `user_id` VARCHAR(191) NOT NULL,
  `option` VARCHAR(191) NOT NULL,
  `value` VARCHAR(191) NOT NULL,
  primary key(`id`)
);
