-- Your SQL goes here
CREATE TABLE status(
  `id` INTEGER AUTO_INCREMENT NOT NULL,
  `guild_id` VARCHAR(191) NOT NULL,
  `user_id` VARCHAR(191) NOT NULL,
  `since_online` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  primary key(`id`)
);
