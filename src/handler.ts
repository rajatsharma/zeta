import { IncomingMessage } from "http";
import { WebSocket } from "ws";
import { clients, WsClient } from "./state.js";

const INDEX_HTML = `
Use the JavaScript console to interact using websockets

sock  = new WebSocket("ws://127.0.0.1:8080/ws")
sock.addEventListener("message", console.log)
sock.addEventListener("open", () => sock.send("ping"))
`;

export function handleUpgrade(
  req: IncomingMessage,
  ws: WebSocket,
  address: string,
): void {
  const client: WsClient = { ws, address };
  clients.set(address, client);

  ws.on("close", () => {
    clients.delete(address);
  });

  ws.on("message", (data) => {
    // handle incoming messages if needed
  });
}

export function handleRequest(req: IncomingMessage, res: { end: (data: string) => void }): boolean {
  const url = new URL(req.url || "/", `http://${req.headers.host}`).pathname;

  if (url === "/") {
    res.end(INDEX_HTML);
    return true;
  }

  return false;
}
