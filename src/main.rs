// src/main.rs
use rusqlite::{Connection, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "SQLite CLI")]
#[command(about = "メモを管理するCLIツール", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { content: String },
    List,
    Update { id: i32, content: String },
    Delete { id: i32 },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = Connection::open("database.sqlite")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memos (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        [],
    )?;

    match cli.command {
        Commands::Add { content } => {
            conn.execute("INSERT INTO memos (content) VALUES (?1)", [&content])?;
            println!("追加されました！");
        }
        Commands::List => {
            let mut stmt = conn.prepare("SELECT id, content FROM memos")?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
            })?;

            for row in rows {
                let (id, content) = row?;
                println!("[{}] {}", id, content);
            }
        }
        Commands::Update { id, content } => {
            conn.execute("UPDATE memos SET content = ?1 WHERE id = ?2", [&content, &id.to_string()])?;
            println!("更新されました！");
        }
        Commands::Delete { id } => {
            conn.execute("DELETE FROM memos WHERE id = ?1", [&id.to_string()])?;
            println!("削除されました！");
        }
    }

    Ok(())
}
