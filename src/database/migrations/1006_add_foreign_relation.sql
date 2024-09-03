
ALTER TABLE "files" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "chats" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "chat_file"
ADD CONSTRAINT "fk_chat_id"
FOREIGN KEY ("chat_id")
REFERENCES "chats" ("id")
ON DELETE CASCADE;

