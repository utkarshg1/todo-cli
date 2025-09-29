# 📋 Todo CLI

A simple command-line todo application written in **Rust**, using:

* [`clap`](https://docs.rs/clap/latest/clap/) for argument parsing
* [`rusqlite`](https://docs.rs/rusqlite/latest/rusqlite/) for SQLite database handling

This tool allows you to manage your tasks directly from the terminal.

---

## 🚀 Features

* Add new todo items
* List all todos (with filters for completed or pending)
* Mark todos as complete
* Delete todos
* Update todo descriptions
* Stores tasks in a local SQLite database (`~/.local/share/todo-cli/todos.db` by default)

---

## 📦 Installation

### Prerequisites

* Rust toolchain (install via [rustup](https://rustup.rs/))

### Build from source

```bash
git clone https://github.com/yourusername/todo-cli.git
cd todo-cli
cargo build --release
```

The compiled binary will be available at:

```
target/release/todo
```

You can move it to your PATH, e.g.:

```bash
mv target/release/todo ~/.local/bin/
```

---

## 📂 Database Location

By default, the database is created at:

```
~/.local/share/todo-cli/todos.db
```

You can override this with the `--database` option:

```bash
todo --database ./mytasks.db add "Buy groceries"
```

---

## 📖 Usage

### General help

```bash
todo --help
```

### Add a todo

```bash
todo add "Finish Rust project"
```

### List todos

```bash
todo list
```

List only completed:

```bash
todo list --completed
```

List only pending:

```bash
todo list --pending
```

### Mark a todo as complete

```bash
todo complete 1
```

### Delete a todo

```bash
todo delete 2
```

### Update a todo description

```bash
todo update 3 "Review pull requests"
```

---

## 📝 Example Output

```
📋 Todo List:
──────────────────────────────────────────────
[ ] #1  Finish Rust project
[✓] #2  Buy groceries
──────────────────────────────────────────────
Total: 2 items
```

---

## 📜 License

MIT License. See [LICENSE](LICENSE) for details.
