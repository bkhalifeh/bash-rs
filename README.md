# Bash-RS

A lightweight, educational **command-line shell** written in Rust, implementing core Unix-like commands with a clean and extensible design.

---

## Features

- **Interactive prompt**: `username@hostname:current_directory$ `
- **Built-in commands**:
  - `pwd` – Print working directory
  - `whoami` – Display current username
  - `hostname` – Show device name
  - `cd <dir>` – Change directory
  - `ls [path]` – List directory contents with icons (📁 for dirs, 📄 for files)
  - `cat <file>` – Display file contents
  - `mkdir <dir>` – Create directory (recursively)
  - `rm <file>` – Remove a file
  - `rmdir <dir>` – Remove directory (and all contents)
  - `clear` – Clear the terminal screen (cross-platform)
  - `exit` – Exit the shell

---

## Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (stable toolchain)

---

## Installation

1. **Clone or download** this project.
2. Add dependencies to `Cargo.toml`:

```toml
[dependencies]
whoami = "1.5"
```

> This crate is used to get the current username and device name.

3. Place the provided Rust code in `src/main.rs`.

---

## Build & Run

```bash
cargo run
```

To build for release:

```bash
cargo build --release
```

Then run the binary:

```bash
./target/release/bash-rs
```

---

## Example Usage

```text
alice@my-laptop:/home/alice$ pwd
/home/alice
alice@my-laptop:/home/alice$ ls
📁 projects
📄 README.md
alice@my-laptop:/home/alice$ cat README.md
# My Project
...
alice@my-laptop:/home/alice$ mkdir new_folder
alice@my-laptop:/home/alice$ cd new_folder
alice@my-laptop:/home/alice/new_folder$ pwd
/home/alice/new_folder
alice@my-laptop:/home/alice/new_folder$ exit
```

---

## Project Structure

```
rust-mini-shell/
├── Cargo.toml
└── src/
    └── main.rs
```

---

## Supported Commands Summary

| Command    | Syntax        | Description             |
| ---------- | ------------- | ----------------------- |
| `pwd`      | `pwd`         | Print current directory |
| `whoami`   | `whoami`      | Show current user       |
| `hostname` | `hostname`    | Show device name        |
| `cd`       | `cd <path>`   | Change directory        |
| `ls`       | `ls [path]`   | List files/folders      |
| `cat`      | `cat <file>`  | Print file content      |
| `mkdir`    | `mkdir <dir>` | Create directory        |
| `rm`       | `rm <file>`   | Delete file             |
| `rmdir`    | `rmdir <dir>` | Delete directory        |
| `clear`    | `clear`       | Clear screen            |
| `exit`     | `exit`        | Quit the shell          |

> **Note**: Only one argument is supported per command (except `ls` defaulting to `.`).

---

## Extending the Shell

To add new commands:

1. Define a handler function:

   ```rust
   fn command_echo(args: &[String]) -> Result<(), std::io::Error> {
       if args.len() > 1 {
           println!("{}", args[1..].join(" "));
       }
       Ok(())
   }
   ```

2. Register it in `main()`:
   ```rust
   command_handlers.insert("echo".to_owned(), command_echo);
   ```

---

## Limitations

- No piping, redirection, or background processes.
- No tab completion or command history.
- Limited argument parsing (whitespace only).
- `cd` only accepts one path argument.
- No error messages on invalid commands/paths (silent fail in some cases).

> Ideal for learning Rust, systems programming, and shell internals.
