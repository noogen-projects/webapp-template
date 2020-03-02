CREATE TABLE `users` (
  `id` int unsigned NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` varchar(1024) NOT NULL,
  UNIQUE (`name`)
);