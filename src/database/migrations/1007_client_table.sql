CREATE TABLE "clients" (
  "id" uuid PRIMARY KEY,
  "login" varchar unique,
  "password" varchar
);