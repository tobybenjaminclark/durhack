### Chat API

| Field                                             | Type          | Description |
|---------------------------------------------------|---------------|-------------|
| **SEND_CHAT(Character: `String`, Message: `String`)** | → `[MESSAGE]` | Send a player message to a specific character and get the updated conversation history. |

**`MESSAGE` object structure**

| Key | Type      | Description |
|-----|-----------|-------------|
| `content` | `String`  | The text of the message. |
| `is_player` | `Boolean` | `true` if the message was sent by the player; `false` if from the character. |
