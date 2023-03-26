use sqlx::{
    self as sql_x,
    postgres::{PgPoolOptions, PgRow},
    FromRow, Row,
};

#[derive(Debug, FromRow)]
struct Ticket {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), sql_x::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432")
        .await?;

    // CREATE TABLE
    sql_x::query(
        r#"
        CREATE TABLE IF NOT EXISTS ticket (
            id bigserial,
            name text
        );"#,
    )
    .execute(&pool)
    .await?;

    // CREATE Record
    let _: (i64,) = sql_x::query_as("insert into ticket (name) values ($1) returning id")
        .bind("a new ticket")
        .fetch_one(&pool)
        .await?;

    // Select all
    let rows = sql_x::query("SELECT * FROM ticket")
        .fetch_all(&pool)
        .await?;
    let result = rows
        .iter()
        .map(|r| format!("{} - {}", r.get::<i64, _>("id"), r.get::<String, _>("name")))
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", result);

    // Select query with map() (build the ticket manually)
    let select_query = sql_x::query("SELECT id, name FROM ticket");
    let tickets = select_query
        .map(|row: PgRow| Ticket {
            id: row.get("id"),
            name: row.get("name"),
        })
        .fetch_all(&pool)
        .await?;
    println!("{:#?}", tickets);

    // Select query_as (using derive from FromRow)
    let select_query = sql_x::query_as::<_, Ticket>("SELECT id, name FROM ticket");
    let tickets = select_query.fetch_all(&pool).await?;
    println!("{:#?}", tickets);

    Ok(())
}
