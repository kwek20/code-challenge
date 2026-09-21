use std::{collections::HashMap, fs::File};

use csv::Reader;

use crate::{Account, Client, ClientId, Result, TransactionRecord};

#[derive(Debug, Clone)]
pub struct System {
    clients: HashMap<ClientId, Client>,
    accounts: Vec<Account>,
}

impl System {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            accounts: vec![],
        }
    }

    pub async fn to_output(&self) -> Result<SystemOutput> {
        
    }

    pub async fn ingest(&mut self, reader: Reader<File>) -> Result<()> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<TransactionRecord>(1);
        tokio::spawn(async move {
            // Move reader over here for clarity
            let mut reader = reader;
            let mut iterator = reader.deserialize();

            if let Some(Ok(result)) = iterator.next() {
                if tx.send(result).await.is_err() {
                    // We closed somehow, end execution
                    return;
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
        client.process(&record).await?;
        Ok(())
    }

    pub fn get_or_create_client(&mut self, client_id: u16) -> &mut Client {
        self.clients.entry(client_id).or_insert(Client::new(client_id))
    }
}
