use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result, params};
use std::env;
use std::path::PathBuf;

/// A simple CLI todo application
#[derive(Parser)]
#[command(name = "todo")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the database file (defaults to ~/.local/share/todo-cli/todos.db)
    #[arg(short, long)]
    database: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        /// Description of the todo item
        description: String,
    },
    /// List all todo items
    List {
        /// Show only completed items
        #[arg(short, long)]
        completed: bool,

        /// Show only pending items
        #[arg(short, long)]
        pending: bool,
    },
    /// Mark a todo item as complete
    Complete {
        /// ID of the todo item to complete
        id: i64,
    },
    /// Delete a todo item
    Delete {
        /// ID of the todo item to delete
        id: i64,
    },
    /// Update a todo item's description
    Update {
        /// ID of the todo item to update
        id: i64,
        /// New description
        description: String,
    },
}

struct Todo {
    id: i64,
    description: String,
    completed: bool,
}

fn init_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

fn add_todo(conn: &Connection, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO todos (description, completed) VALUES (?1, 0)",
        params![description],
    )?;
    let id = conn.last_insert_rowid();
    println!("✓ Added todo #{}: {}", id, description);
    Ok(())
}

fn list_todos(conn: &Connection, completed: bool, pending: bool) -> Result<()> {
    let query = match (completed, pending) {
        (true, false) => {
            "SELECT id, description, completed FROM todos WHERE completed = 1 ORDER BY id"
        }
        (false, true) => {
            "SELECT id, description, completed FROM todos WHERE completed = 0 ORDER BY id"
        }
        _ => "SELECT id, description, completed FROM todos ORDER BY id",
    };

    let mut stmt = conn.prepare(query)?;
    let todos = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut count = 0;
    println!("\n📋 Todo List:");
    println!("{}", "─".repeat(60));

    for todo in todos {
        let todo = todo?;
        let status = if todo.completed { "✓" } else { " " };
        let style = if todo.completed { "\x1b[90m" } else { "" };
        let reset = if todo.completed { "\x1b[0m" } else { "" };

        println!(
            "{}[{}] #{:<3} {}{}",
            style, status, todo.id, todo.description, reset
        );
        count += 1;
    }

    println!("{}", "─".repeat(60));
    println!("Total: {} items\n", count);
    Ok(())
}

fn complete_todo(conn: &Connection, id: i64) -> Result<()> {
    let affected = conn.execute("UPDATE todos SET completed = 1 WHERE id = ?1", params![id])?;

    if affected > 0 {
        println!("✓ Marked todo #{} as complete", id);
    } else {
        println!("✗ Todo #{} not found", id);
    }
    Ok(())
}

fn delete_todo(conn: &Connection, id: i64) -> Result<()> {
    let affected = conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;

    if affected > 0 {
        println!("✓ Deleted todo #{}", id);
    } else {
        println!("✗ Todo #{} not found", id);
    }
    Ok(())
}

fn update_todo(conn: &Connection, id: i64, description: &str) -> Result<()> {
    let affected = conn.execute(
        "UPDATE todos SET description = ?1 WHERE id = ?2",
        params![description, id],
    )?;

    if affected > 0 {
        println!("✓ Updated todo #{}: {}", id, description);
    } else {
        println!("✗ Todo #{} not found", id);
    }
    Ok(())
}

fn get_default_db_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let data_dir = PathBuf::from(home).join(".local/share/todo-cli");

    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&data_dir).ok();

    data_dir.join("todos.db")
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Use custom path or default to ~/.local/share/todo-cli/todos.db
    let db_path = cli.database.unwrap_or_else(get_default_db_path);

    // Open database connection
    let conn = Connection::open(&db_path)?;
    init_database(&conn)?;

    match cli.command {
        Commands::Add { description } => {
            add_todo(&conn, &description)?;
        }
        Commands::List { completed, pending } => {
            list_todos(&conn, completed, pending)?;
        }
        Commands::Complete { id } => {
            complete_todo(&conn, id)?;
        }
        Commands::Delete { id } => {
            delete_todo(&conn, id)?;
        }
        Commands::Update { id, description } => {
            update_todo(&conn, id, &description)?;
        }
    }

    Ok(())
}
