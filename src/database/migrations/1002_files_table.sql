CREATE TABLE "files" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "name" varchar,
  "file_id" varchar,
  "file_path" varchar,
  "file_content" varchar,
  "created_at" timestamp
);