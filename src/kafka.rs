use crate::{
    models::{
        kafka::{KafkaCommand, KafkaEvent},
        sample::Sample,
        state::WebState,
    },
    services,
};
use anyhow::Result;
use futures_util::StreamExt;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::time::Duration;

#[derive(Clone)]
pub struct EventBus {
    producer: FutureProducer,
    topic: String,
}

impl EventBus {
    pub fn new(brokers: &str, topic: &str) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;
        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }

    pub async fn sample_created(&self, s: Sample) -> Result<()> {
        self.send(KafkaEvent::SampleCreated { sample: s }).await
    }
    pub async fn sample_updated(&self, s: Sample) -> Result<()> {
        self.send(KafkaEvent::SampleUpdated { sample: s }).await
    }
    pub async fn sample_deleted(&self, id: i64) -> Result<()> {
        self.send(KafkaEvent::SampleDeleted { id }).await
    }

    async fn send(&self, ev: KafkaEvent) -> Result<()> {
        let payload = serde_json::to_vec(&ev)?;
        tracing::info!(
            "publishing to topic={}, bytes={}, event={:?}",
            self.topic,
            payload.len(),
            ev
        );

        let dr = self
            .producer
            .send(
                FutureRecord::to(&self.topic)
                    .key("sample")
                    .payload(&payload),
                Duration::from_secs(5),
            )
            .await;

        match dr {
            Ok(_) => Ok(()),
            Err((e, _msg)) => {
                tracing::error!(error=?e, topic=%self.topic, "delivery failed");
                Err(anyhow::anyhow!(e))
            }
        }
    }
}

pub async fn run_command_consumer(
    db: SqlitePool,
    brokers: String,
    topic: String,
    event_bus: EventBus,
) -> Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "sample-app")
        .set("bootstrap.servers", &brokers)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .set("debug", "broker,protocol,topic,metadata")
        .create()?;

    consumer.subscribe(&[&topic])?;

    let mut stream = consumer.stream();

    while let Some(msg) = stream.next().await {
        if let Ok(m) = msg {
            if let Some(Ok(payload)) = m.payload_view::<str>() {
                if let Ok(cmd) = serde_json::from_str::<KafkaCommand>(payload) {
                    handle_command(&db, cmd, &event_bus).await.ok();
                }
            }
        }
    }

    Ok(())
}

async fn handle_command(db: &SqlitePool, cmd: KafkaCommand, event_bus: &EventBus) -> Result<()> {
    let state = WebState {
        db: db.clone(),
        events: event_bus.clone(),
    };

    tracing::info!("event received, event={:?}", cmd);

    match cmd {
        KafkaCommand::CreateSample { input, user_id } => {
            services::sample::create_sample(&state, input, user_id).await?;
        }
        KafkaCommand::UpdateSample { id, input, .. } => {
            services::sample::update_sample_by_id(&state, input, id).await?;
        }
        KafkaCommand::DeleteSample { id } => {
            services::sample::delete_sample_by_id(&state, id).await?;
        }
    }
    Ok(())
}

pub async fn setup_kafka(db: Pool<Sqlite>) -> Result<EventBus> {
    let brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "127.0.0.1:19092".into());
    let events_topic =
        std::env::var("KAFKA_EVENTS_TOPIC").unwrap_or_else(|_| "sample-events".into());
    let commands_topic =
        std::env::var("KAFKA_COMMANDS_TOPIC").unwrap_or_else(|_| "sample-commands".into());
    let event_bus = EventBus::new(&brokers, &events_topic)?;

    let db_for_consumer = db.clone();
    let eb_for_consumer = event_bus.clone();

    tokio::spawn(async move {
        if let Err(e) =
            run_command_consumer(db_for_consumer, brokers, commands_topic, eb_for_consumer).await
        {
            tracing::error!(?e, "command consumer crashed");
        }
    });

    Ok(event_bus)
}
