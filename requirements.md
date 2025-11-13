# 0-Shell Project: Team Workflow Breakdown

## 👤 Member 1: Shell Core & Parser

### Day 1: Setup & Planning
- [ ] Create project structure
- [ ] Define Command/ParsedCommand types in `types.rs`
- [ ] Design parser interface
- [ ] Coordinate with Member 3 on error types

### Day 2-3: Main Shell Loop
- [ ] Implement `main.rs` REPL loop
- [ ] Handle stdin reading with BufReader
- [ ] Display prompt `$ `
- [ ] Implement Ctrl+D (EOF) handling
- [ ] Connect to parser module

### Day 3-4: Command Parser
- [ ] Implement `parser.rs`
- [ ] Tokenize input string
- [ ] Split command and arguments
- [ ] Handle quoted strings
- [ ] Parse flags (`-l`, `-a`, `-F`, `-r`)
- [ ] Return ParsedCommand struct

### Day 5: Signal Handling
- [ ] Test basic shell loop
- [ ] Implement Ctrl+C handling (bonus)
- [ ] Add signal hooks

### Day 6-7: Integration
- [ ] Test with Member 3's executor
- [ ] Handle edge cases (empty input)
- [ ] Test error message display
- [ ] Refine prompt display

### Day 8: Polish
- [ ] Add command history (bonus)
- [ ] Implement colorized prompt (bonus)
- [ ] Add auto-completion (bonus)

---

## 👤 Member 2: File System Commands

### Day 1: Setup & Planning
- [ ] Review `std::fs` documentation
- [ ] Create commands module structure
- [ ] Define function signatures
- [ ] Coordinate with Member 3 on error handling

### Day 2: Basic Commands
- [ ] Implement `pwd` command
  - Use `std::env::current_dir()`
- [ ] Implement `mkdir` command
  - Use `std::fs::create_dir()`
- [ ] Test basic functionality

### Day 3: File Operations
- [ ] Implement `cat` command
  - Use `std::fs::read_to_string()`
- [ ] Implement `cp` command
  - Use `std::fs::copy()`
- [ ] Handle file not found errors

### Day 4: Move & Remove
- [ ] Implement `mv` command
  - Use `std::fs::rename()`
- [ ] Implement `rm` command
  - Handle `-r` flag for recursive
  - Use `remove_file()` vs `remove_dir_all()`

### Day 5-6: ls Command (Complex)
- [ ] Implement basic `ls`
  - Use `std::fs::read_dir()`
- [ ] Implement `-a` flag (show hidden files)
- [ ] Implement `-l` flag (long format)
  - Use `std::fs::metadata()`
  - Format permissions, size, date
- [ ] Implement `-F` flag (classify)
  - Add `/` for directories
  - Add `*` for executables

### Day 7: Integration & Testing
- [ ] Test all file operations
- [ ] Handle edge cases
- [ ] Test permission errors
- [ ] Validate against real `ls` output

### Day 8: Polish
- [ ] Optimize performance
- [ ] Add colorized output (bonus)
- [ ] Refine error messages

---

## 👤 Member 3: Process Commands & Integration

### Day 1: Setup & Planning
- [ ] Create `error.rs` module
- [ ] Define custom error types
- [ ] Implement `From` traits for std errors
- [ ] Define Result type aliases
- [ ] Coordinate interfaces with team

### Day 2: Simple Commands
- [ ] Implement `echo` command
  - Handle multiple arguments
  - Handle quoted strings
- [ ] Implement `exit` command
  - Return exit signal

### Day 3: cd Command
- [ ] Implement `cd` command
  - Use `std::env::set_current_dir()`
  - Handle `~` (home directory)
  - Handle `..` (parent directory)
  - Handle `.` (current directory)
  - Handle `cd` with no args (goes to $HOME)

### Day 4: Command Executor
- [ ] Create `executor.rs`
- [ ] Implement command dispatcher
- [ ] Match command names
- [ ] Route to appropriate function
- [ ] Handle unknown commands
- [ ] Return "Command '<name>' not found" error

### Day 5: Error Handling
- [ ] Implement error display traits
- [ ] Format user-friendly messages
- [ ] Handle "permission denied"
- [ ] Handle "file not found"
- [ ] Handle "invalid arguments"
- [ ] Test error propagation

### Day 6-7: Integration Testing
- [ ] Create `integration_tests.rs`
- [ ] Test all commands end-to-end
- [ ] Test command chaining
- [ ] Test error scenarios
- [ ] Test with Members 1 & 2 modules
- [ ] Fix integration bugs
- [ ] Validate against requirements

### Day 8: Bonus & Polish
- [ ] Implement environment variables (bonus)
- [ ] Add `help` command (bonus)
- [ ] Implement command chaining with `;` (bonus)
- [ ] Final testing and documentation
- [ ] Code review and cleanup

---

## 📋 Integration Checkpoints

### Checkpoint 1 (End of Day 1)
**All Members Meet**
- Finalize shared types in `types.rs`
- Agree on error handling strategy
- Review function signatures
- Set up Git workflow

### Checkpoint 2 (End of Day 3)
**Members 1 & 3 Sync**
- Test parser → executor connection
- Verify ParsedCommand structure works
- Test basic command flow

### Checkpoint 3 (End of Day 5)
**All Members Meet**
- Member 2 demos file commands
- Member 1 demos shell loop
- Member 3 integrates everything
- Identify and assign bug fixes

### Checkpoint 4 (End of Day 7)
**All Members Meet**
- Final integration testing
- Code review
- Plan Day 8 polish/bonus work
- Assign documentation tasks

---

## 🎯 Critical Dependencies

### Member 1 → Member 3
- `types.rs` must be complete before Member 3 can build executor
- Parser output format must match executor input

### Member 2 → Member 3
- All command functions must return consistent Result types
- Error types must be defined early

### Member 3 → Members 1 & 2
- Error types must be ready Day 1
- Executor interface must be clear for integration

---

## 🚀 Success Criteria by Day

- **Day 3**: Basic shell loop works, can parse and echo commands
- **Day 5**: All individual commands work in isolation
- **Day 6**: Full integration complete, all commands work in shell
- **Day 7**: All bugs fixed, edge cases handled
- **Day 8**: Polished, documented, bonus features added