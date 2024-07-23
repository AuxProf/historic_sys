CREATE TABLE "users" (
  "id" uuid PRIMARY KEY,
  "email" varchar unique,
  "password" varchar
);