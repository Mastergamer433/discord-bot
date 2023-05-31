-- Your SQL goes here
CREATE TABLE balance(
  `id` INTEGER AUTO_INCREMENT,
  `user_id` VARCHAR(191) NOT NULL,
  `points` INTEGER NOT NULL,
  primary key(`id`)
);
