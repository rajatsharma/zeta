import http from "http";
import { WebSocketServer } from "ws";
import { handleUpgrade, handleRequest } from "./handler.js";
import { startListener } from "./listener.js";

const PORT = 8080;

async function main(): Promise<void> {
  const server = http.createServer((req, res) => {
    if (!handleRequest(req, res)) {
      res.writeHead(404);
      res.end("Not Found");
    }
  });

  const wss = new WebSocketServer({ noServer: true });
  wss.on("connection", (ws, req) => {
    const address = (req.socket as any).remoteAddress || "unknown";
    handleUpgrade(req, ws, address);
  });

  server.on("upgrade", (req, socket, head) => {
    wss.handleUpgrade(req, socket, head, (ws) => {
      wss.emit("connection", ws, req);
    });
  });

  await startListener();

  server.listen(PORT, "127.0.0.1", () => {
    console.log(`Listening on 127.0.0.1:${PORT}`);
  });
}

main().catch(console.error);
