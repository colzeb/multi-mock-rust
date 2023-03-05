pub async fn db_connection() -> sqlx::SqlitePool {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://test.sqlite")
        .await
        .unwrap()
}
