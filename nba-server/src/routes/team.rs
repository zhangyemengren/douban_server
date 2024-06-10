use crate::data::{AppState, Team};
use axum::{extract::State, Json};

pub async fn get_team(State(AppState { pool, .. }): State<AppState>) -> Json<Vec<Team>> {
    let teams = sqlx::query_as!(
        Team,
        r#"
            SELECT * FROM team
            "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap_or(vec![]);
    teams.into()
}
