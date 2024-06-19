use crate::data::{AppState, Conference, Team};
use axum::{extract::{State, Path}, Json};

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

pub async fn get_team_by_conference(Path(c): Path<Conference>, State(AppState { pool, .. }): State<AppState>) -> Json<Vec<Team>> {
    let teams = sqlx::query_as!(
        Team,
        r#"
            SELECT * FROM team where conference = ?
            "#,
            "east"
    )
        .fetch_all(&pool)
        .await
        .unwrap_or(vec![]);
    teams.into()
}
