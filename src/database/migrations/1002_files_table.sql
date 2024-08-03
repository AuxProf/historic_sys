CREATE TABLE "files" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "name" varchar,
  "file_id" varchar,
  "created_at" timestamp
);