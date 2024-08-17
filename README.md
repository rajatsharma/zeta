# Lectro

> Websocket server which listens to postgres changes and notify frontend clients via websockets

## Usage

- Start the server using `cargo run`.
- Open page `http://127.0.0.1:8080` in the browser.
- Open Console and paste this code

```js
sock  = new WebSocket("ws://127.0.0.1:8080/ws")
sock.addEventListener("message", console.log)
sock.addEventListener("open", () => sock.send("ping"))
```

- Now connect to postgres using any client to: `postgres://postgres:postgres@localhost:5432/postgres`
- Run this query for `postgres` DB
```sql
NOTIFY test_notifications, 'hello';
```
- Check the browser console, it should be logged with the message from Websocket
