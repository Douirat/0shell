# 0-Shell Project: Team Task Division

## 🎯 Command Implementation Responsibilities

### 👤 **Member 1: Shell Core & Parser**
**Does NOT implement commands** - Builds the infrastructure

**Core Responsibilities:**
- Main shell REPL loop (Read-Eval-Print-Loop)
- Command parsing and tokenization
- User input handling
- Signal handling (Ctrl+D, Ctrl+C)

**Deliverables:**
- `main.rs` - Shell loop that displays `$ ` prompt
- `parser.rs` - Parses user input into command + arguments + flags
- `types.rs` - Shared data structures (Command, ParsedCommand, etc.)

---

### 👤 **Member 2: File System Commands**
**Implements 7 commands** - All file/directory operations

#### Commands to Implement:

1. **`pwd`** - Print working directory
   ```rust
   // Use: std::env::current_dir()
   ```

2. **`ls`** - List directory contents
   ```rust
   // Use: std::fs::read_dir(), std::fs::metadata()
   // Flags: -l (long format), -a (show hidden), -F (classify)
   ```

3. **`cat`** - Display file contents
   ```rust
   // Use: std::fs::read_to_string()
   ```

4. **`cp`** - Copy files
   ```rust
   // Use: std::fs::copy()
   ```

5. **`rm`** - Remove files/directories
   ```rust
   // Use: std::fs::remove_file(), std::fs::remove_dir_all()
   // Flags: -r (recursive for directories)
   ```

6. **`mv`** - Move/rename files
   ```rust
   // Use: std::fs::rename()
   ```

7. **`mkdir`** - Create directories
   ```rust
   // Use: std::fs::create_dir()
   ```

**Deliverables:**
- `commands/fs_commands.rs` - All 7 file system commands
- `commands/ls.rs` - Dedicated module for complex `ls` implementation
- `utils/file_utils.rs` - Helper functions for formatting, permissions, etc.

---

### 👤 **Member 3: Process Commands & Integration**
**Implements 3 commands + orchestrates everything**

#### Commands to Implement:

1. **`echo`** - Print arguments to stdout
   ```rust
   // Simply print all arguments
   // Handle: echo "Hello World"
   ```

2. **`cd`** - Change directory
   ```rust
   // Use: std::env::set_current_dir()
   // Handle: cd /path, cd .., cd ~, cd (no args = go home)
   ```

3. **`exit`** - Exit the shell
   ```rust
   // Signal the shell loop to terminate
   ```

**Core Responsibilities:**
- Command executor/dispatcher (routes commands to correct functions)
- Error handling system (custom error types)
- Integration testing (make sure all parts work together)

**Deliverables:**
- `commands/process_commands.rs` - echo, cd, exit implementations
- `executor.rs` - Command dispatcher that routes to Member 2's or own commands
- `error.rs` - Custom error types and "Command not found" messages
- `tests/integration_tests.rs` - Test all commands work together

---

## 📊 Command Distribution Summary

| Member | Commands | Count | Complexity |
|--------|----------|-------|------------|
| Member 1 | *Infrastructure only* | 0 | High (Shell loop) |
| Member 2 | pwd, ls, cat, cp, rm, mv, mkdir | 7 | High (ls is complex) |
| Member 3 | echo, cd, exit | 3 | Medium (+ integration) |

**Total Commands: 10**

---

## 🔄 How Commands Flow Through the System

```
User Input: "ls -l /home"
    ↓
[Member 1: Parser]
    → Parses into: Command="ls", Args=["-l", "/home"]
    ↓
[Member 3: Executor]
    → Routes to Member 2's ls_command()
    ↓
[Member 2: ls_command()]
    → Reads directory, formats output
    ↓
Output displayed to user
```

---

## 📝 Detailed Task Breakdown

### Member 1: Shell Core & Parser

#### Week Plan:
**Day 1-2: Shell Loop**
- [ ] Create main.rs with infinite loop
- [ ] Display `$ ` prompt
- [ ] Read user input with `std::io::stdin()`
- [ ] Handle Ctrl+D (EOF) to exit
- [ ] Call parser and executor

**Day 3-4: Parser**
- [ ] Split input into tokens
- [ ] Identify command name (first token)
- [ ] Separate flags (starts with `-`)
- [ ] Separate arguments
- [ ] Handle quoted strings: `echo "Hello World"`

**Day 5-7: Polish & Integration**
- [ ] Test with Member 3's executor
- [ ] Handle empty input gracefully
- [ ] Add error display
- [ ] Bonus: Command history, Ctrl+C handling

---

### Member 2: File System Commands

#### Week Plan:
**Day 1-2: Simple Commands**
- [ ] **pwd** - Get and print current directory
- [ ] **mkdir** - Create directory

**Day 3: File Reading/Writing**
- [ ] **cat** - Read and display file
- [ ] **cp** - Copy file from source to destination

**Day 4: File Operations**
- [ ] **mv** - Rename/move file
- [ ] **rm** - Delete file
- [ ] **rm -r** - Recursively delete directory

**Day 5-6: Complex ls Command**
- [ ] **ls** (basic) - List files in directory
- [ ] **ls -a** - Show hidden files (starting with `.`)
- [ ] **ls -l** - Long format (permissions, size, date, name)
- [ ] **ls -F** - Add `/` to directories, `*` to executables

**Day 7: Testing & Polish**
- [ ] Test all commands with various inputs
- [ ] Handle errors (file not found, permission denied)
- [ ] Format output to match Unix ls

---

### Member 3: Process Commands & Integration

#### Week Plan:
**Day 1: Error Handling Setup**
- [ ] Create error.rs with custom error types
- [ ] Define ShellError enum
- [ ] Implement error messages

**Day 2: Simple Commands**
- [ ] **echo** - Print all arguments
- [ ] **exit** - Set flag to terminate shell

**Day 3: cd Command**
- [ ] **cd <path>** - Change to specified directory
- [ ] **cd ..** - Go to parent directory
- [ ] **cd ~** - Go to home directory
- [ ] **cd** (no args) - Go to home directory

**Day 4: Command Executor**
- [ ] Create executor.rs
- [ ] Match command name
- [ ] Route to correct function:
  - "echo" → echo_command()
  - "cd" → cd_command()
  - "pwd" → pwd_command() [Member 2]
  - "ls" → ls_command() [Member 2]
  - etc.
- [ ] Unknown command → "Command 'xyz' not found"

**Day 5-7: Integration & Testing**
- [ ] Connect Member 1's parser to executor
- [ ] Connect executor to Member 2's commands
- [ ] Write integration tests for all 10 commands
- [ ] Test error scenarios
- [ ] Final bug fixes

---

## ✅ Definition of Done

### Each Command Must:
- [ ] Work with correct arguments
- [ ] Handle errors gracefully
- [ ] Display appropriate error messages
- [ ] Match Unix behavior
- [ ] Not crash the shell

### Integration Complete When:
- [ ] All 10 commands work in the shell
- [ ] Unknown commands show error message
- [ ] Ctrl+D exits the shell
- [ ] No external binaries are called
- [ ] Error handling is consistent

---

## 🚨 Critical Rules

**Member 2 & 3 - When Implementing Commands:**
- ❌ **DO NOT** use `std::process::Command` to spawn `/bin/ls`, `/bin/cat`, etc.
- ✅ **DO** use `std::fs`, `std::env`, and `std::io` directly
- ✅ **DO** return `Result<(), ShellError>` from all command functions
- ✅ **DO** handle all errors (file not found, permission denied, etc.)

**Example - WRONG:**
```rust
// ❌ This calls external binary - NOT ALLOWED
std::process::Command::new("ls").spawn();
```

**Example - CORRECT:**
```rust
// ✅ This uses Rust's std library directly
for entry in std::fs::read_dir(".")? {
    println!("{}", entry?.file_name().to_string_lossy());
}
```

---

## 📞 Communication & Coordination

### Daily Standups (5 minutes):
- What did I complete yesterday?
- What am I working on today?
- Any blockers?

### Integration Points:
- **Day 1**: All members agree on types.rs structure
- **Day 3**: Member 1 & 3 test parser → executor
- **Day 5**: Member 3 tests executor → Member 2's commands
- **Day 6**: Full integration test by Member 3

---

## 🎁 Bonus Features (If Time Permits)

After all core commands work:
- Command history (up/down arrows)
- Tab auto-completion
- Colorized output
- Command chaining with `;`
- Pipes `|`
- I/O redirection `>`, `<`
- Environment variables `$HOME`, `$PATH`
- Custom `help` command