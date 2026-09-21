use std::{collections::HashMap, fs::File, io::Write};

use csv::Reader;

use crate::{Client, ClientId, Result, TransactionRecord};

#[derive(Debug, Clone)]
pub struct System {
    clients: HashMap<ClientId, Client>,
}

impl System {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub async fn write(&self, writer: impl Write) -> Result<()> {
        let mut csv_writer = csv::Writer::from_writer(writer);

        self.clients.values().for_each(|c| {
            let res = csv_writer.serialize(c.record());
            if let Err(e) = res {
                tracing::error!("Failed to write client {} due to {e:?}", c.id())
            }
        });
        Ok(())
    }

    pub async fn ingest(&mut self, reader: Reader<File>) -> Result<()> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<TransactionRecord>(1);
        tokio::spawn(async move {
            // Move reader over here for clarity
            let mut reader = reader;
            let mut iterator = reader.deserialize();

            while let Some(s) = iterator.next() {
                if let Ok(record) = s {
                    tracing::error!("record: {:?}", record);
                    if tx.send(record).await.is_err() {
                        // We closed somehow, end execution
                        return;
                    }
                }
            }
        });

        while let Some(tx) = rx.recv().await {
            self.process(tx).await?;
        }

        Ok(())
    }

    pub async fn process(&mut self, record: TransactionRecord) -> Result<()> {
        let client = self.get_or_create_client(record.client);
        client.process(record).await?;
        Ok(())
    }

    pub fn get_or_create_client(&mut self, client_id: u16) -> &mut Client {
        self.clients.entry(client_id).or_insert(Client::new(client_id))
    }
}
