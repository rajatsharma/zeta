import { Client } from "pg";
import { clients } from "./state.js";

export async function startListener(): Promise<void> {
  const pgClient = new Client({
    host: "localhost",
    port: 5432,
    database: "postgres",
    user: "postgres",
    password: "postgres",
  });

  await pgClient.connect();
  console.log("PostgreSQL connected");

  await pgClient.query("LISTEN test_notifications");

  pgClient.on("notification", (notification) => {
    console.log(`Received notification: ${notification.channel} - ${notification.payload}`);

    for (const client of clients.values()) {
      if (client.ws.readyState === 1) {
        client.ws.send(notification.payload ?? "" );
      }
    }
  });

  pgClient.on("error", (err) => {
    console.error("PostgreSQL error:", err);
  });
}
