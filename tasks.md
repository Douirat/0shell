sequenceDiagram
    actor User
    participant Shell as 0-Shell Main Loop
    participant Parser as Command Parser
    participant Executor as Command Executor
    participant FS as File System API
    participant Process as Process API

    User->>Shell: Launch ./0-shell
    activate Shell
    
    loop Shell Loop
        Shell->>User: Display prompt "$ "
        User->>Shell: Enter command
        
        alt Ctrl+D (EOF)
            Shell->>User: Exit gracefully
            deactivate Shell
        else Regular command
            Shell->>Parser: Parse input string
            activate Parser
            Parser->>Parser: Tokenize arguments
            Parser->>Parser: Extract command & flags
            Parser-->>Shell: Return parsed command
            deactivate Parser
            
            Shell->>Executor: Execute(command, args)
            activate Executor
            
            alt Built-in command
                alt cd
                    Executor->>FS: std::env::set_current_dir()
                    FS-->>Executor: Result
                else ls
                    Executor->>FS: std::fs::read_dir()
                    FS-->>Executor: Directory entries
                    Executor->>User: Format & display
                else pwd
                    Executor->>FS: std::env::current_dir()
                    FS-->>Executor: Current path
                    Executor->>User: Display path
                else cat/cp/rm/mv/mkdir
                    Executor->>FS: Corresponding std::fs operation
                    FS-->>Executor: Result
                else echo
                    Executor->>User: Print arguments
                else exit
                    Executor->>Shell: Signal exit
                end
                
                alt Success
                    Executor-->>Shell: Ok
                else Error
                    Executor->>User: Display error message
                    Executor-->>Shell: Err
                end
                
            else Unknown command
                Executor->>User: "Command '<name>' not found"
                Executor-->>Shell: Err
            end
            
            deactivate Executor
        end
    end