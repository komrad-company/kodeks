use sqlx::PgPool;

use crate::Alert;
use crate::Error;
use crate::alert::AlertRow;

impl AlertRow {
    pub(crate) async fn insert(pool: &PgPool, alert: &Alert) -> Result<(), Error> {
        let row = AlertRow::from(alert);
        sqlx::query(
            "INSERT INTO alerts (id, rule_id, title, level, event, triggered_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(row.id)
        .bind(&row.rule_id)
        .bind(&row.title)
        .bind(&row.level)
        .bind(&row.event)
        .bind(row.triggered_at)
        .execute(pool)
        .await?;
        Ok(())
    }
}
