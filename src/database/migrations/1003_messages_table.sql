CREATE TABLE "messages" (
  "id" uuid PRIMARY KEY,
  "chat_id" uuid,
  "role" varchar,
  "content" varchar,
  "created_at" timestamp
);