use types::command::*;
use std::os::unix::fs::PermissionsExt;
/*
# =========================================================
# Custom LS behavior focused on: -a, -l, -F
# =========================================================
#
# 1. `-a` : Show all files including hidden files
#    - Hidden files start with a dot `.`
#    - Special entries:
#        .   -> current directory
#        ..  -> parent directory
#    - Without -a, these entries are not shown
#
# 2. `-l` : Long listing format
#    - Displays detailed information per file:
#        [0] File type and permissions
#        [1] Number of hard links
#        [2] Owner name
#        [3] Group name
#        [4] File size (in bytes)
#        [5] Last modification date and time
#        [6] File name
#    - Example:
#        drwxr-xr-x  3 bdouirat talent 4096 Jan 24 22:55 errors/
#        ^   ^       ^       ^      ^      ^         ^
#        |   |       |       |      |      |         +-- file name (+ type indicator if -F)
#        |   |       |       |      |      +-- modification date
#        |   |       |       |      +-- size in bytes
#        |   |       |       +-- group name
#        |   |       +-- owner name
#        |   +-- number of hard links
#        +-- file type & permissions
#
# 3. `-F` : Append type indicators to file names
#    - Helps distinguish file types quickly
#        /  -> directory
#        *  -> executable
#        @  -> symbolic link
#        |  -> FIFO / pipe
#        =  -> socket
#        (nothing) -> regular file
#    - Often combined with -l to produce: long listing + type indicators
#
# 4. Combining `-a`, `-l`, and `-F`:
#    - `ls -alF` (or `ls -aFl`) will:
#        1. List all files including hidden ones
#        2. Show detailed information (permissions, owner, size, date)
#        3. Append type indicators for quick identification
#
# Example output of `ls -alF`:
# drwxr-xr-x  8 bdouirat talent 4096 Jan 24 23:27 ./       # current directory
# drwxr-xr-x  5 bdouirat talent 4096 Jan 24 22:55 ../      # parent directory
# -rw-r--r--  1 bdouirat talent  558 Jan 24 22:55 Cargo.lock   # regular file
# drwxr-xr-x  3 bdouirat talent 4096 Jan 24 22:55 errors/       # directory
# =========================================================
*/
use std::fs;
use std::path::Path;
 use std::path::PathBuf;

pub fn ls<'a>(command: &Command) {
    let state = &command.state;

    // Determine directory to list
    let dir = if command.args.is_empty() {
        state.cwd.borrow().clone() // default to current working directory
    } else {
        let path_str = command.args[0].clone();
        if Path::new(&path_str).is_absolute() {
            PathBuf::from(path_str)
        } else {
            state.cwd.borrow().join(path_str)
        }
    };

    // Read directory entries
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("ls: cannot access '{}': {}", dir.display(), err);
            return;
        }
    };

    // Flags
    let show_all = command.flags.contains(&Flag::A);
    let long_list = command.flags.contains(&Flag::L);
    let list_types = command.flags.contains(&Flag::F);

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            // Skip hidden files if -a is not set
            if !show_all && file_name.starts_with(".") {
                continue;
            }

            // Long format
            if long_list {
                let metadata = match entry.metadata() {
                    Ok(meta) => meta,
                    Err(_) => continue,
                };
                let file_type = if metadata.is_dir() { "d" } else { "-" };
                let size = metadata.len();
                println!("{} {:>8} {}", file_type, size, file_name);
            } else if list_types {
                // Append / for directories, * for executables
                let metadata = match entry.metadata() {
                    Ok(meta) => meta,
                    Err(_) => continue,
                };
                let suffix = if metadata.is_dir() {
                    "/"
                } else if metadata.permissions().mode() & 0o111 != 0 {
                    "*" // executable
                } else {
                    ""
                };
                println!("{}{}", file_name, suffix);
            } else {
                println!("{}", file_name);
            }
        }
    }
}


// extract the flags from the command:
pub fn handle_ls_flags(args: &Vec<Flag> )->Result<LsFlag, String> {
    let mut flags = LsFlag::default();
    for flag in args {
        match flag{
            Flag::A => flags.list_all = true,
            Flag::L => flags.long_list = true,
            Flag::F => flags.list_types = true,
            Flag::R => todo!(),
        }
    }
    Ok(flags)
}


/*
# =========================================================
# Mini LS pseudocode supporting: -a, -l, -F
# =========================================================

# Function: mini_ls(directory, flags)
# Inputs:
#   directory -> path to list
#   flags -> dictionary or struct { a: bool, l: bool, F: bool }
# Outputs: prints file list according to flags

function mini_ls(directory, flags):

    # Step 1: Read all entries in the directory
    # - Use OS API (opendir/readdir in C, read_dir in Rust)
    # - Collect all file/directory names in a list
    entries = read_directory(directory)

    # Step 2: Filter entries based on -a
    # - If -a is not set, skip entries starting with '.' (hidden files)
    # - If -a is set, include all entries including '.' and '..'
    if not flags.a:
        entries = filter_out_hidden(entries)

    # Step 3: Sort entries alphabetically (optional but standard behavior)
    entries.sort()

    # Step 4: For each entry, gather info and print
    for entry in entries:

        # Step 4a: Determine file metadata for -l
        # - Only if -l flag is set
        # - Use system API (stat/lstat) to get:
        #   - File type and permissions
        #   - Number of hard links
        #   - Owner name
        #   - Group name
        #   - File size in bytes
        #   - Last modification date/time
        metadata = None
        if flags.l:
            metadata = get_metadata(entry)

        # Step 4b: Determine type indicator for -F
        # - Only if -F flag is set
        # - Symbols:
        #     / -> directory
        #     * -> executable
        #     @ -> symbolic link
        #     | -> named pipe / FIFO
        #     = -> socket
        #     (nothing) -> regular file
        type_indicator = ""
        if flags.F:
            type_indicator = determine_type_symbol(entry, metadata)

        # Step 4c: Prepare output string
        output = ""
        if flags.l:
            # Format the long listing line:
            # [file type & permissions] [hard links] [owner] [group] [size] [modification date] [file name]
            output += format_long_line(metadata)

        # Append the file name and type indicator
        output += entry.name + type_indicator

        # Step 4d: Print the formatted line
        print(output)

# =========================================================
# Notes / Key points:
# - Order of flags does NOT matter; all combinations of -a, -l, -F should work.
# - Flags can be independently enabled or disabled.
# - Metadata (permissions, owner, group, size, modification time) is only needed if -l is enabled.
# - Type indicator is only needed if -F is enabled.
# - Hidden file filtering is only needed if -a is NOT enabled.
# - Sorting is optional but recommended to match typical ls behavior.
# =========================================================
*/