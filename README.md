# acapair_chat_api
Acapair Chat API

[![Rust Build && Test](https://github.com/Acapair/acapair_chat_api/actions/workflows/rust.yml/badge.svg)](https://github.com/Acapair/acapair_chat_api/actions/workflows/rust.yml)


## Exposed URLs
> ':' means they are variable.

Alive Ping(get): "/"

---

Send Message(post): "/send"

| Body Fields    | Example Values |
| -------- | ------- |
| "room_id"  | "Tahinli's Room -1"    |
| "username" | "Tahinli"     |
| "message"    | "Hi!"    |

---

Receive Message(get): "/receive/:room_id"