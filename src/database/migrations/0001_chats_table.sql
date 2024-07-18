CREATE TABLE "chats" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "title" varchar,
  "created_at" timestamp
);
