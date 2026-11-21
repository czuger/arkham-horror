use sqlx::{AssertSqlSafe, SqlitePool};

use crate::database::DatabaseError;

/// Schema steps, in order. The database's `user_version` counts those
/// already applied. Never edit a released step: add a new one.
const MIGRATIONS: &[&str] = &[include_str!("../migrations/0001_game.sql")];

/// Applies the schema steps the database lacks, in one transaction.
///
/// # Errors
///
/// [`DatabaseError::Newer`] if the database was written by a newer
/// version of the program, [`DatabaseError::Sqlx`] if a step fails.
pub async fn migrate(writer: &SqlitePool) -> Result<(), DatabaseError> {
    let mut tx = writer.begin().await?;
    let found: i64 = sqlx::query_scalar("PRAGMA user_version")
        .fetch_one(&mut *tx)
        .await?;
    let known = MIGRATIONS.len();
    let applied = usize::try_from(found)
        .ok()
        .filter(|&applied| applied <= known)
        .ok_or(DatabaseError::Newer { found, known })?;
    for (step, sql) in MIGRATIONS.iter().enumerate().skip(applied) {
        sqlx::raw_sql(*sql).execute(&mut *tx).await?;
        // A PRAGMA takes no bound parameter; the value is our own integer.
        sqlx::query(AssertSqlSafe(format!("PRAGMA user_version = {}", step + 1)))
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}
