import { WebSocket } from "ws";

export interface WsClient {
  ws: WebSocket;
  address: string;
}

export const clients = new Map<string, WsClient>();
