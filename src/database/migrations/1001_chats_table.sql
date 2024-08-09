CREATE TABLE "chats" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "title" varchar,
  "thread_id" varchar,
  "created_at" timestamp
);
