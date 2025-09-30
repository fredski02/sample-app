use anyhow::Result;
use tracing::error;

use crate::models::{
    sample::{Sample, SampleInput},
    state::WebState,
};

pub async fn create_sample(state: &WebState, input: SampleInput, uid: i64) -> Result<Sample> {
    let sample = sqlx::query_as!(
        Sample,
        r#"
            INSERT INTO samples (name, description, status, created_by)
            VALUES (?, ?, ?, ?)
            RETURNING
                id          AS "id!",
                name        AS "name!",
                description AS "description?",   -- nullable
                status      AS "status!",
                created_at  AS "created_at!",
                updated_at  AS "updated_at!",
                created_by  AS "created_by!"
            "#,
        input.name,
        input.description,
        input.status,
        uid
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    state.events.sample_created(sample.clone()).await.ok();

    Ok(sample)
}

pub async fn get_samples(state: &WebState) -> Vec<Sample> {
    sqlx::query_as!(Sample, r#"SELECT * FROM samples ORDER BY id DESC"#)
        .fetch_all(&state.db)
        .await
        .unwrap_or(vec![])
}

pub async fn get_sample_by_id(state: &WebState, sample_id: &i64) -> Option<Sample> {
    let s = sqlx::query_as!(Sample, r#"SELECT * FROM samples WHERE id = ?"#, sample_id)
        .fetch_one(&state.db)
        .await;
    s.ok()
}

pub async fn update_sample_by_id(state: &WebState, input: SampleInput, uid: i64) -> Result<Sample> {
    let sample = sqlx::query_as!(
        Sample,
        r#"UPDATE samples SET name = ?, description = ?, status = ?, updated_at = datetime('now')
WHERE id = ? RETURNING *"#,
        input.name,
        input.description,
        input.status,
        uid
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    state
        .events
        .sample_updated(sample.clone())
        .await
        .map_err(|e| {
            error!(?e, "failed to publish to sample updated");
            e
        })?;

    Ok(sample)
}

pub async fn delete_sample_by_id(state: &WebState, id: i64) -> Result<()> {
    sqlx::query!("DELETE FROM samples WHERE id = ?", id)
        .execute(&state.db)
        .await?;

    state.events.sample_deleted(id).await?;

    Ok(())
}
