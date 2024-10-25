CREATE TABLE IF NOT EXISTS `users` (
  `trap_id` VARCHAR(32) NOT NULL,
  `traq_uuid` CHAR(36) NOT NULL,
  `is_bot` BOOLEAN,
  `is_expired` BOOLEAN,
  PRIMARY KEY (`trap_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `words` (
  `word_id` BIGINT(8) NOT NULL AUTO_INCREMENT,
  `word_uuid` BINARY(36) NOT NULL,
  `trap_id` VARCHAR(32) NOT NULL,
  `word` VARCHAR(50) NOT NULL,
  `register_time` DATETIME DEFAULT NOW(),
  PRIMARY KEY (`word_id`),
  FOREIGN KEY (`trap_id`) REFERENCES `users`(`trap_id`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `word_excluded_users` (
  `word_id` BIGINT(8) NOT NULL,
  `trap_id` VARCHAR(32) NOT NULL,
  FOREIGN KEY (`word_id`) REFERENCES `words`(`word_id`),
  FOREIGN KEY (`trap_id`) REFERENCES `users`(`trap_id`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `stamps` (
  `stamp_id` BIGINT(8) NOT NULL AUTO_INCREMENT,
  `stamp_uuid` BINARY(36) NOT NULL,
  `trap_id` VARCHAR(32) NOT NULL,
  `word` VARCHAR(50) NOT NULL,
  `register_time` DATETIME DEFAULT NOW(),
  PRIMARY KEY (`stamp_id`),
  FOREIGN KEY (`trap_id`) REFERENCES `users`(`trap_id`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `stamp_excluded_users` (
  `stamp_id` BIGINT(8) NOT NULL,
  `trap_id` VARCHAR(32) NOT NULL,
  FOREIGN KEY (`stamp_id`) REFERENCES `stamps`(`stamp_id`),
  FOREIGN KEY (`trap_id`) REFERENCES `users`(`trap_id`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `polling` (
  `key` INT NOT NULL,
  `last` datetime NOT NULL,
  PRIMARY KEY (`key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
