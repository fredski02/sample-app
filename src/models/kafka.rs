use serde::{Deserialize, Serialize};

use crate::models::sample::{Sample, SampleInput};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KafkaEvent {
    SampleCreated { sample: Sample },
    SampleUpdated { sample: Sample },
    SampleDeleted { id: i64 },
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KafkaCommand {
    CreateSample {
        input: SampleInput,
        user_id: i64,
    },
    UpdateSample {
        id: i64,
        input: SampleInput,
        user_id: i64,
    },
    DeleteSample {
        id: i64,
    },
}
