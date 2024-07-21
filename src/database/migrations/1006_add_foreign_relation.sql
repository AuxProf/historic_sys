
ALTER TABLE "files" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "chats" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "messages" ADD FOREIGN KEY ("chat_id") REFERENCES "chats" ("id");

ALTER TABLE "chat_file" ADD FOREIGN KEY ("chat_id") REFERENCES "chats" ("id");

ALTER TABLE "chat_file" ADD FOREIGN KEY ("file_id") REFERENCES "files" ("id");
