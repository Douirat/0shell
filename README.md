# 0-shell — Project Overview

> A minimalist Unix-like shell built in Rust, using system-level abstractions without relying on external binaries or existing shells.

---

## What Is This Project?

**0-shell** is a standalone, self-contained shell for an embedded Linux environment. It mimics the behavior of a standard Unix shell by implementing core commands from scratch — no `bash`, no `sh`, no external binaries.

Think of it as a micro-[BusyBox](https://busybox.net/): a single binary that understands file navigation, process control, and I/O — all written safely in Rust.

---

## Core Shell Requirements

The shell must:

- Display a `$ ` prompt and block until the user types a command
- Parse input and dispatch to the appropriate built-in handler
- Wait for command completion before returning to the prompt
- Exit cleanly on `Ctrl+D` (EOF)

---

## Commands to Implement (From Scratch)

| Command | Notes |
|---------|-------|
| `echo` | Print arguments to stdout |
| `cd` | Change working directory |
| `ls` | List directory contents; support `-l`, `-a`, `-F` flags |
| `pwd` | Print current working directory |
| `cat` | Read and print file contents |
| `cp` | Copy a file |
| `rm` | Remove file or directory; support `-r` for recursive |
| `mv` | Move or rename a file/directory |
| `mkdir` | Create a new directory |
| `exit` | Exit the shell |

> If a command is not recognized, print: `Command '<name>' not found`

---

## Hard Constraints

- **No external binaries** — do not call `ls`, `cat`, `cp`, etc. from the system
- **No shell delegation** — do not spawn `bash`, `sh`, or any other shell
- **No piping, redirection, or globbing** in the core version (`|`, `>`, `*` are optional bonuses)
- Behavior must align with **Unix conventions**
- Code must follow **good coding practices** (clean structure, error handling, meaningful names)

---

## Implementation Strategy in Rust

| Task | Rust API |
|------|----------|
| Read user input | `std::io::stdin().read_line()` |
| Change directory | `std::env::set_current_dir()` |
| List directory | `std::fs::read_dir()` |
| File metadata | `std::fs::metadata()` / `DirEntry::metadata()` |
| Read file | `std::fs::read_to_string()` or `File::open()` |
| Copy file | `std::fs::copy()` |
| Remove file | `std::fs::remove_file()` / `remove_dir_all()` |
| Move/rename | `std::fs::rename()` |
| Create directory | `std::fs::create_dir()` / `create_dir_all()` |
| Current dir | `std::env::current_dir()` |

---

## Error Handling

The shell must **never crash** on user errors. All commands should handle:

- Missing arguments
- Non-existent paths
- Permission errors
- Invalid flags

Use Rust's `Result` and `match`/`?` patterns to propagate and display errors gracefully.

---

## Bonus Features (Optional)

Implementing any of these will be scored as bonus:

- **`Ctrl+C` (SIGINT) handling** — shell continues running instead of exiting
- **Auto-completion** — tab-complete file names and commands
- **Command history** — navigate previous commands with arrow keys
- **Dynamic prompt** — show current directory, e.g. `~/projects/0-shell $`
- **Colorized output** — color-code directories, errors, executables
- **Command chaining** — run multiple commands with `;`
- **Pipes** — `cmd1 | cmd2`
- **I/O Redirection** — `>`, `<`
- **Environment variables** — expand `$HOME`, `$PATH`, etc.
- **`help` command** — document all built-in commands

---

## Example Session

```
student$ ./0-shell
$ cd /dev
$ pwd
/dev
$ ls -l
total 0
crw------- 1 root root 10, 58 Feb 5 09:21 acpi_thermal_rel
...
$ something
Command 'something' not found
$ echo "Hello There"
Hello There
$ exit
student$
```

---

## Evaluation Criteria

| Criterion | What It Means |
|-----------|---------------|
| **Functionality** | Each command behaves like its standard Unix equivalent |
| **Stability** | No panics or crashes on bad input or edge cases |
| **Code Quality** | Readable, modular, well-named code with proper error handling |

---

## Key Resources

- [Rust `std::fs` docs](https://doc.rust-lang.org/std/fs/)
- [Rust `std::process` docs](https://doc.rust-lang.org/std/process/)
- [Linux man pages](https://man7.org/linux/man-pages/) (`man 2 open`, `man 2 execve`, etc.)
- [Unix Shell — Wikipedia](https://en.wikipedia.org/wiki/Unix_shell)
- [BusyBox](https://busybox.net/)

---

## Project Structure (Suggested)

```
0-shell/
├── Cargo.toml
└── src/
    ├── main.rs          # Shell loop, input parsing
    └── commands/
        ├── mod.rs       # Command dispatcher
        ├── echo.rs
        ├── cd.rs
        ├── ls.rs
        ├── pwd.rs
        ├── cat.rs
        ├── cp.rs
        ├── rm.rs
        ├── mv.rs
        └── mkdir.rs
```

---

*Project by Agiel OTCHOUN & Bennacer Douirat — Zone01 Oujda*
