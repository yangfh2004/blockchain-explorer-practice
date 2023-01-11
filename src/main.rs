use std::io;

use blockchain_explorer::Block;
use blockchain_explorer::Service;

fn main() {
    let mut service = blockchain_explorer::ServiceImpl::new();
    let scenario: Scenario =
        serde_json::from_str(&io::read_to_string(io::stdin()).expect("Failed to read stdin"))
            .expect("Failed to deserialize input");

    for event in scenario.play_out(&mut service) {
        println!("{:?}", event);
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Scenario {
    blocks: Vec<Block>,
    queries: Vec<String>,
}

impl Scenario {
    fn play_out<S: Service>(&self, service: &mut S) -> Vec<Event> {
        let insertion_events: Vec<Event> = self
            .blocks
            .iter()
            .map(|block| match service.ingest_block(block) {
                Ok(()) => Event::BlockIngestion(block.block_id.to_string()),
                Err(e) => Event::IngestionError(block.block_id.to_string(), e.to_string()),
            })
            .collect();

        let query_events: Vec<Event> = self
            .queries
            .iter()
            .map(|query| match service.get_balance(query) {
                Ok(result) => Event::QueryResult(query.to_string(), format!("{:?}", result)),
                Err(e) => Event::QueryError(query.to_string(), e.to_string()),
            })
            .collect();

        [insertion_events, query_events]
            .into_iter()
            .flatten()
            .collect()
    }
}

#[derive(Debug)]
enum Event {
    BlockIngestion(String),
    IngestionError(String, String),
    QueryResult(String, String),
    QueryError(String, String),
}
