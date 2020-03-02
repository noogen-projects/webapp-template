CREATE DATABASE webapp CHARACTER SET utf8;
CREATE USER IF NOT EXISTS 'webapp'@'localhost' IDENTIFIED BY 'webapp';
GRANT ALL PRIVILEGES ON webapp.* TO 'webapp'@'localhost';