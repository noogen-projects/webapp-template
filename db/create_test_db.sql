CREATE DATABASE webapp_test CHARACTER SET utf8;
CREATE USER IF NOT EXISTS 'webapp_test'@'localhost' IDENTIFIED BY 'webapp_test';
GRANT ALL PRIVILEGES ON webapp_test.* TO 'webapp_test'@'localhost';