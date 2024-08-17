use crate::state::{Message, SharedState};
use sqlx::postgres::PgListener;
use tokio_postgres::Error;

pub async fn listener(state: SharedState) -> Result<(), Error> {
    let mut listener = PgListener::connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap();

    tokio::task::spawn(async move {
        let _ = listener.listen("test_notifications").await;
        loop {
            let message = listener.recv().await;

            match message {
                Ok(m) => {
                    println!("{}", m.channel());
                    let state = state.read().await;
                    let clients = &state.clients;

                    for (_, sender) in clients {
                        let _ = sender.send(Message::Text(m.payload().to_string())).await;
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    });

    Ok(())
}
