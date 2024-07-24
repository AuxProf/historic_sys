CREATE TABLE "users" (
  "id" uuid PRIMARY KEY,
  "login" varchar unique,
  "password" varchar
);