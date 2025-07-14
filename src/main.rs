use anyhow::Result;
use dialoguer::{Input, Select};
use serde::Serialize;
use sqlx::{mysql::MySqlPoolOptions, FromRow};
use std::env;

#[derive(Debug, FromRow, Serialize)]
struct Book {
    id: i32,
    title: String,
    author: String,
    year: i16,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Change the URL or export DATABASE_URL before running
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@localhost/mylib".to_string());

    // Create the connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    loop {
        let options = [
            "Add", "List", "Search", "Update", "Delete",
            "Sort by Title", "Sort by Year", "Export CSV", "Exit",
        ];
        let choice = Select::new()
            .with_prompt("Choose an action")
            .items(&options)
            .default(0)
            .interact()?;

        match choice {
            0 => add(&pool).await?,
            1 => list(&pool).await?,
            2 => search(&pool).await?,
            3 => update(&pool).await?,
            4 => delete(&pool).await?,
            5 => list_sorted(&pool, "title").await?,
            6 => list_sorted(&pool, "year").await?,
            7 => export_csv(&pool).await?,
            8 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

async fn add(pool: &sqlx::MySqlPool) -> Result<()> {
    let id: i32     = Input::new().with_prompt("ID").interact_text()?;
    let title: String  = Input::new().with_prompt("Title").interact_text()?;
    let author: String = Input::new().with_prompt("Author").interact_text()?;
    let year: i16      = Input::new().with_prompt("Year").interact_text()?;

    sqlx::query("INSERT INTO books (id, title, author, year) VALUES (?, ?, ?, ?)")
        .bind(id)
        .bind(title)
        .bind(author)
        .bind(year)
        .execute(pool)
        .await?;

    println!("Added.\n");
    Ok(())
}

async fn list(pool: &sqlx::MySqlPool) -> Result<()> {
    let rows = sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(pool)
        .await?;
    print_books(&rows);
    Ok(())
}

async fn list_sorted(pool: &sqlx::MySqlPool, field: &str) -> Result<()> {
    let query = format!("SELECT * FROM books ORDER BY {}", field);
    let rows: Vec<Book> = sqlx::query_as(&query).fetch_all(pool).await?;
    print_books(&rows);
    Ok(())
}

fn print_books(rows: &[Book]) {
    println!("\n{:<6} {:<30} {:<20} {}", "ID", "Title", "Author", "Year");
    for b in rows {
        println!("{:<6} {:<30} {:<20} {}", b.id, b.title, b.author, b.year);
    }
    println!();
}

async fn search(pool: &sqlx::MySqlPool) -> Result<()> {
    let id: i32 = Input::new().with_prompt("ID to search").interact_text()?;

    match sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
    {
        Some(b) => println!("Found: {:?}\n", b),
        None => println!("Not found.\n"),
    }
    Ok(())
}

async fn update(pool: &sqlx::MySqlPool) -> Result<()> {
    let id: i32 = Input::new().with_prompt("ID to update").interact_text()?;
    if let Some(b) = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
        .bind(id).fetch_optional(pool).await? {

        let title  = Input::new().with_prompt("Title").default(b.title).interact_text()?;
        let author = Input::new().with_prompt("Author").default(b.author).interact_text()?;
        let year: i16 = Input::new().with_prompt("Year").default(b.year).interact_text()?;

        sqlx::query("UPDATE books SET title = ?, author = ?, year = ? WHERE id = ?")
            .bind(title)
            .bind(author)
            .bind(year)
            .bind(id)
            .execute(pool).await?;

        println!("Updated.\n");
    } else {
        println!("Not found.\n");
    }
    Ok(())
}
async fn delete(pool: &sqlx::MySqlPool) -> Result<()> {
    let id: i32 = Input::new().with_prompt("ID to delete").interact_text()?;

    sqlx::query("DELETE FROM books WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    println!("Deleted if existed.\n");
    Ok(())
}

async fn export_csv(pool: &sqlx::MySqlPool) -> Result<()> {
    let rows = sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(pool)
        .await?;

    let mut wtr = csv::Writer::from_path("books.csv")?;
    for b in rows {
        wtr.serialize(b)?;
    }
    wtr.flush()?;
    println!("CSV exported to books.csv\n");
    Ok(())
}
