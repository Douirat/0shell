# 🐚 Mini Shell

A minimalist Unix shell written in Rust, supporting essential commands, flags, and I/O redirections.

---

## Features

- **10 built-in commands** : `echo`, `pwd`, `cd`, `ls`, `cat`, `cp`, `mv`, `mkdir`, `rm`, `exit`
- **Flags** : `-l`, `-a`, `-F` for `ls` — `-r` for `rm`
- **I/O redirections** : `>` (overwrite), `>>` (append)
- **Multiple commands** via `;`, `&&`, `||`
- **Quote handling** : single `'...'` and double `"..."` quotes, backslash escaping
- **Persistent state** : current directory shared across all commands via `Rc<RefCell<PathBuf>>`

---

## Project Structure

```
mini_shell/
├── src/main.rs              # Entry point — prompt loop
├── types/                   # Shared data structures (Command, State, Flag…)
├── parser/                  # Tokenizer + command parser
├── executer/
│   ├── src/executer.rs      # Dispatch + redirection handling
│   └── executers/src/       # One file per command
```

The data flow is: **user input → parser → executer → output**

---

## Architecture

### State (`types/state.rs`)

A shared `State` struct holds the shell's runtime context:

| Field | Type | Description |
|-------|------|-------------|
| `user` | `String` | Current Unix username |
| `host` | `String` | Machine hostname |
| `home` | `PathBuf` | Home directory (constant) |
| `cwd`  | `RefCell<PathBuf>` | Current working directory (mutable) |

`RefCell` enables **interior mutability** — the `cwd` can be updated by `cd` through a shared `Rc<State>` reference, without requiring `&mut`.

### Parser (`parser/`)

Three stages:
1. **`split_commands`** — splits input on `;` / `&&` / `||`, quote-aware
2. **`tokenize`** — splits a command into tokens, handles quotes, escapes, and isolates `>` / `>>` / `<`
3. **`parse_flags_args_redirections`** — classifies each token as a `Flag`, `Redirection`, or argument

### Executer (`executer/`)

Loops over parsed commands. If a redirection is present, it opens the target file and routes output there. Otherwise it dispatches to the matching command function.

---

## Commands

| Command | Description | Supported flags |
|---------|-------------|-----------------|
| `echo [args]` | Print arguments to stdout | — |
| `pwd` | Print current working directory | — |
| `cd [path]` | Change directory (`~` or empty → home) | — |
| `ls [path]` | List directory contents | `-l` (long), `-a` (hidden), `-F` (type suffix) |
| `cat <file...>` | Concatenate and display file contents | — |
| `cp <src> <dest>` | Copy a file | — |
| `mv <src> <dest>` | Move or rename a file | — |
| `mkdir <dir...>` | Create one or more directories | — |
| `rm <file...>` | Remove files or directories | `-r` (recursive) |
| `exit` | Exit the shell | — |

---

## Usage

### Build & Run

```bash
cd mini_shell
cargo run || cargo r
```

### Example session

```bash
$ pwd
/home/user

$ mkdir test && cd test
$ echo "hello" > hello.txt
$ cat hello.txt
hello

$ ls -la
drwxr-xr-x  2 user user     40 Feb 18 10:00 .
drwxr-xr-x 15 user user   4096 Feb 18 10:00 ..
-rw-r--r--  1 user user      6 Feb 18 10:00 hello.txt*

$ cd .. && rm -r test
$ exit
Exiting the shell. Goodbye!
```

---

## Dependencies

```toml
[dependencies]
hostname = "0.3"
users    = "0.11"
chrono   = "0.4"
```

- **`hostname`** — retrieve machine name for the prompt
- **`users`** — resolve UID/GID to usernames in `ls -l`
- **`chrono`** — format file modification timestamps in `ls -l`

---

## Key Rust Concepts Used

- **`Rc<RefCell<T>>`** — shared ownership + interior mutability for the shell state
- **Lifetimes (`'a`)** — `Command<'a>` borrows `State` without copying it
- **Pattern matching** — used extensively for command dispatch, error handling, and file type detection
- **`canonicalize()`** — resolves relative paths and `..` in `cd`, and verifies existence on disk

---

## Authors

- **Bennacer DOUIRAT** — `ls`, `cd`, `pwd`, `mkdir`, `rm`  
- **Agiel OTCHOUN** — `cat`, `cp`, `mv`, `echo`, `exit`
