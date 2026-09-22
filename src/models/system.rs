use std::{collections::HashMap, fs::File, io::Write};

use csv::Reader;

use crate::{Client, ClientId, Result, TransactionRecord};

#[derive(Debug, Clone)]
pub struct System {
    clients: HashMap<ClientId, Client>,
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

impl System {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    /// Serializes the all the clients to the writer
    /// Could be done async but as its the final step of this challenge, left as is.
    pub async fn write(&self, writer: impl Write) -> Result<()> {
        let mut csv_writer = csv::Writer::from_writer(writer);

        // Hashmap is unordered, so the output can change order each run.
        // As per instructions, this is ok, and hashMap is faster than BTreeMap.
        self.clients.values().for_each(|c| {
            let res = csv_writer.serialize(c.record());
            if let Err(e) = res {
                tracing::error!("Failed to write client {} due to {e:?}", c.id())
            }
        });

        csv_writer.flush()?;
        Ok(())
    }

    /// Take in all records from the reader.
    /// Failing ot parse a row into a TransactionRecord will log and skip.
    pub async fn ingest(&mut self, reader: Reader<File>) -> Result<()> {
        // Keep 1 channel to ensure order remains the same
        let (tx, mut rx) = tokio::sync::mpsc::channel::<TransactionRecord>(1);
        tokio::spawn(async move {
            let mut reader = reader;
            let mut iterator = reader.deserialize();

            // Allow manual flattening for readability and type inference on iterator
            #[allow(clippy::manual_flatten)]
            for s in &mut iterator {
                match s {
                    Ok(record) => {
                        tracing::error!("{:?}", record);
                        if tx.send(record).await.is_err() {
                            // We closed somehow, end execution
                            return;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse CSV row due to {:?}, skipping", e);
                    }
                }
            }
        });

        while let Some(tx) = rx.recv().await {
            self.process(tx).await?;
        }

        Ok(())
    }

    /// Process a TransactionRecord.
    /// If the client related to the record doesnt exist, it gets created first.
    pub async fn process(&mut self, record: TransactionRecord) -> Result<()> {
        let client = self.get_or_create_client(record.client);
        client.process(record).await?;
        Ok(())
    }

    pub fn get_or_create_client(&mut self, client_id: u16) -> &mut Client {
        self.clients.entry(client_id).or_insert(Client::new(client_id))
    }
}
