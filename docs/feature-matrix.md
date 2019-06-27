# Backend feature matrix

Features that various pastebin servers (backends) support.

Key:

- `x`: supported
- `-`: server api supports but pc doesn't yet
- ` ` (blank): not supported by server or backend

| backend       | title | expiry | password | email notification | auth | poster name | syntax / filetype   |
| -------       | ----- | ------ | -------- | ------------------ | ---- | ----------- | -----------------   |
| bpaste        |       | x      |          |                    |      |             | x                   |
| dpaste        |       | x      |          |                    |      |             | x                   |
| dpaste_com    | x     | x      |          |                    |      | x           | x                   |
| fiche         |       |        |          |                    |      |             |                     |
| haste         |       |        |          |                    |      |             | -                   |
| ix            |       |        |          |                    | x    |             | x                   |
| modern_paste  | x     | x      | x        |                    | x    |             | x                   |
| onetimesecret |       | x      | x        | x                  | x    |             |                     |
| paste_rs      |       |        |          |                    |      |             | -                   |
| pipfi         |       |        |          |                    |      |             | x<sup>1</sup>       |
| sprunge       |       |        |          |                    |      |             | x                   |
| ubuntu        |       | x      |          |                    |      | x           | x                   |
| vpaste        |       |        |          |                    |      |             | -                   |

<sup>1</sup>autodetected
