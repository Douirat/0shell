use types::command::*;
use types::state::*;
use std::fs::DirEntry;
use std::os::unix::fs::FileTypeExt;
use std::fmt;
use std::os::unix::fs::MetadataExt; // uid, gid, nlink, blocks, mtime, rdev, mode
use std::path::*;
use std::time::{UNIX_EPOCH, Duration};
use users::{get_user_by_uid, get_group_by_gid};
use std::fs::read_dir;
use term_grid::{Grid, GridOptions, Direction, Filling, Cell};

// ── Data structures ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct LsNames(Vec<String>);

#[derive(Debug, Clone)]
pub struct FileEntry {
    /// 10-char permission string including leading type char, e.g. "drwxr-xr-x".
    pub permissions: String,
    /// '+' if the file has ACL/xattrs, ' ' otherwise.
    pub acl: char,
    /// -F indicator char: '/' dir, '@' symlink, '|' fifo, '=' socket, '*' exec, '\0' none.
    /// Only injected into output when the caller requested -F.
    pub sign: char,
    /// True when this entry is "." or ".." so we can exclude it from block totals.
    pub is_dotdot: bool,
    pub links: u64,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub blocks: u64,
    pub mtime: i64,
    pub name: String,
    /// Some((major, minor)) for char/block devices; None for everything else.
    pub rdev: Option<(u64, u64)>,
    /// Some("target") for symlinks when -l is active; None otherwise.
    pub link_target: Option<String>,
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            permissions: String::new(),
            acl:         ' ',
            sign:        '\0',
            is_dotdot:   false,
            links:       0,
            uid:         0,
            gid:         0,
            size:        0,
            blocks:      0,
            mtime:       0,
            name:        String::new(),
            rdev:        None,
            link_target: None,
        }
    }
}

// ── Display (-l format) ───────────────────────────────────────────────────────

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let user = get_user_by_uid(self.uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| self.uid.to_string());

        let group = get_group_by_gid(self.gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| self.gid.to_string());

        // char/block devices: show "major, minor" not byte size.
        // GNU ls formats this as "%3u, %3u" inside the same width as the size column.
        let size_field = match self.rdev {
            Some((maj, min)) => format!("{:>3},{:>6}", maj, min),
            None             => format!("{:>8}", self.size),
        };

        // Symlinks: "name -> target". No sign — the arrow makes the type obvious.
        // Non-symlinks use self.name as-is (sign already appended by list() if -F).
        let name_field = match &self.link_target {
            Some(target) => format!("{} -> {}", self.name, target),
            None         => self.name.clone(),
        };

        write!(
            f,
            "{}{} {:>3} {:>8} {:>8} {} {} {}",
            self.permissions, // 10 chars: type + rwxrwxrwx
            self.acl,         // '+' or ' '
            self.links,
            user,
            group,
            size_field,
            format_time(self.mtime),
            name_field,
        )
    }
}

// ── FileEntry construction ────────────────────────────────────────────────────

impl FileEntry {
    pub fn new() -> FileEntry {
        FileEntry::default()
    }

    /// Build from a `DirEntry` (directory scan).
    /// Uses `symlink_metadata` so symlinks report as 'l', not their target type.
    pub fn get_entry_from_entry(&mut self, entry: DirEntry) -> Result<Self, std::io::Error> {
        self.name = entry.file_name().to_str().unwrap_or("<invalid utf8>").to_string();
        let path = entry.path();
        // symlink_metadata = lstat: does NOT follow the symlink.
        let meta = std::fs::symlink_metadata(&path)?;
        self.fill_from_meta(&meta, &path)?;
        Ok(self.clone())
    }

    /// Build from an explicit path (used for "." and "..").
    pub fn get_entry_from_path(&mut self, path: &Path) -> Result<&mut FileEntry, std::io::Error> {
        // symlink_metadata = lstat: correct for "." and ".." which are always dirs.
        let meta = std::fs::symlink_metadata(path)?;
        self.fill_from_meta(&meta, path)?;
        Ok(self)
    }

    /// Shared metadata extraction — called by both constructors above.
    fn fill_from_meta(
        &mut self,
        meta: &std::fs::Metadata,
        path: &Path,
    ) -> Result<(), std::io::Error> {
        let ft   = meta.file_type();
        let mode = meta.mode(); // raw Unix mode from MetadataExt

        // ── leading type character (one of: - d l b c p s ?) ─────────────────
        let type_char = if ft.is_file()         { '-' }
                   else if ft.is_dir()          { 'd' }
                   else if ft.is_symlink()      { 'l' }
                   else if ft.is_block_device() { 'b' }
                   else if ft.is_char_device()  { 'c' }
                   else if ft.is_fifo()         { 'p' }
                   else if ft.is_socket()       { 's' }
                   else                         { '?' };

        // ── -F indicator ────────────────────────────────────────────────────────
        self.sign = if ft.is_dir()                          { '/' }
               else if ft.is_symlink()                      { '@' }
               else if ft.is_fifo()                         { '|' }
               else if ft.is_socket()                       { '=' }
               else if ft.is_file() && (mode & 0o111 != 0) { '*' }
               else                                          { '\0' };

        // ── permission string ─────────────────────────────────────────────────
        self.permissions = format!("{}{}", type_char, perms_to_string(mode));

        // ── ACL / xattr check ─────────────────────────────────────────────────
        self.acl = acl_indicator(path);

        // ── standard stat fields ──────────────────────────────────────────────
        self.links  = meta.nlink();
        self.uid    = meta.uid();
        self.gid    = meta.gid();
        self.size   = meta.len();
        self.blocks = meta.blocks();
        self.mtime  = meta.mtime();

        // ── major/minor (char/block devices only) ─────────────────────────────
        self.rdev = if ft.is_block_device() || ft.is_char_device() {
            let raw = meta.rdev();
            Some((dev_major(raw), dev_minor(raw)))
        } else {
            None
        };

        // ── symlink target via read_link() ─────────────────────────────────────
        self.link_target = if ft.is_symlink() {
            std::fs::read_link(path)
                .ok()
                .map(|t| t.to_string_lossy().into_owned())
        } else {
            None
        };

        Ok(())
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn ls(command: &Command) {
    // Initialise the C locale from the environment so strcoll() sorts
    // exactly like GNU ls (respects LC_COLLATE / LANG).
    init_locale();
    if command.args.is_empty() {
        if let Err(e) = list(&command.state, &command.flags, ".") {
            eprintln!("ls: .: {e}");
        }
    } else {
        for arg in &command.args {
            if let Err(e) = list(&command.state, &command.flags, arg) {
                eprintln!("ls: {arg}: {e}");
            }
        }
    }
}

// ── list ──────────────────────────────────────────────────────────────────────

pub fn list(
    state:  &State,
    flags:  &Vec<Flag>,
    arg:    &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let show_all = flags.contains(&Flag::A);
    let long     = flags.contains(&Flag::L);
    let classify = flags.contains(&Flag::F);

    // Resolve the target directory / file.
    // We join the raw arg onto the cwd and canonicalize to resolve any ".."
    // components.  We do NOT canonicalize through symlinks at the final
    // component — symlink_metadata() called later will see the link itself.
    let cwd    = state.cwd.borrow().clone();
    let target = cwd.join(arg);
    // canonicalize resolves ".." and makes the path absolute, but it also
    // follows symlinks.  For the common case (pointing at a dir) this is fine.
    // When the arg IS a symlink we handle it below via symlink_metadata.
    let target = target.canonicalize()?;

    // ── If target is not a directory, just list that one entry and return ──
    {
        let target_meta = std::fs::symlink_metadata(&target)?;
        if !target_meta.is_dir() {
            let mut f = FileEntry::new();
            f.get_entry_from_path(&target)?;
            f.name = target
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| arg.to_string());
            // For non-symlinks, append the -F sign to the name directly.
            if classify && f.sign != '\0' && f.link_target.is_none() {
                f.name.push(f.sign);
            }
            if long {
                println!("{}", f);
            } else {
                println!("{}", f.name);
            }
            return Ok(());
        }
    }

    // ── Collect directory entries ─────────────────────────────────────────
    let mut entries: Vec<FileEntry> = Vec::new();

    // "." and ".." are only shown with -a, exactly like GNU ls.
    if show_all {
        let mut dot = FileEntry::new();
        dot.get_entry_from_path(&target)?;
        dot.name      = ".".to_string();
        dot.is_dotdot = true;
        entries.push(dot);

        if target != Path::new("/") {
            let parent = target.parent().unwrap_or(Path::new("/")).to_path_buf();
            let mut dotdot = FileEntry::new();
            dotdot.get_entry_from_path(&parent)?;
            dotdot.name      = "..".to_string();
            dotdot.is_dotdot = true;
            entries.push(dotdot);
        }
    }

    // All other entries in the directory.
    for entry in read_dir(&target)?.flatten() {
        let mut f = FileEntry::new();
        f.get_entry_from_entry(entry)?;
        entries.push(f);
    }

    // ── Hide dot-files when -a is absent ─────────────────────────────────
    // (. and .. were never added in this branch, so this only hides .hidden files)
    if !show_all {
        entries.retain(|f| !f.name.starts_with('.'));
    }

    // ── Sort ──────────────────────────────────────────────────────────────
    // "." always first, ".." always second.
    // Everything else sorted with strcoll(3) — the same function GNU ls uses —
    // which respects the process locale (LC_COLLATE) exactly as the shell does.
    entries.sort_by(|a, b| {
        let rank = |e: &FileEntry| -> u8 {
            if !e.is_dotdot { return 2; }
            if e.name == "." { 0 } else { 1 }
        };
        let ra = rank(a);
        let rb = rank(b);
        if ra != rb {
            return ra.cmp(&rb);
        }
        strcoll(&a.name, &b.name)
    });

    // ── Block total (computed BEFORE -F mutates names) ────────────────────
    // Kernel reports 512-byte blocks; GNU ls shows 1 KiB units → divide by 2.
    // GNU ls always includes "." blocks in the total regardless of -a, but
    // never includes "..". When -a is absent "." was never pushed to entries,
    // so we fetch its block count directly from the target path metadata.
    // GNU ls total = st_blocks of ALL shown entries (including . and ..) / 2.
    // Kernel reports 512-byte blocks; ls shows 1 KiB units → divide by 2.
    // When -a is absent, . and .. are not in entries, so we fetch them directly.
    let dot_blocks: u64 = if show_all {
        0 // already in entries, counted below
    } else {
        std::fs::symlink_metadata(&target)
            .map(|m| MetadataExt::blocks(&m))
            .unwrap_or(0)
    };
    let dotdot_blocks: u64 = if show_all {
        0 // already in entries, counted below
    } else if target != Path::new("/") {
        target.parent()
            .and_then(|p| std::fs::symlink_metadata(p).ok())
            .map(|m| MetadataExt::blocks(&m))
            .unwrap_or(0)
    } else {
        0
    };
    let entry_blocks: u64 = entries.iter().map(|f| f.blocks).sum();
    let total_blocks: u64 = (dot_blocks + dotdot_blocks + entry_blocks) / 2;

    // ── Apply -F sign to non-symlink names ───────────────────────────────
    // Symlinks are handled entirely inside Display (sign goes before "->").
    // For everything else we append the sign char directly to the name now.
    if classify {
        for f in &mut entries {
            if f.sign != '\0' && f.link_target.is_none() {
                f.name.push(f.sign);
            }
        }
    }

    // ── Output ────────────────────────────────────────────────────────────
    if long {
        println!("total {}", total_blocks);
        for f in &entries {
            println!("{}", f);
        }
    } else {
        let names: Vec<String> = entries.iter().map(|f| f.name.clone()).collect();
        println!("{}", LsNames(names));
    }

    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Compares two filenames using the locale-aware strcoll(3), exactly as GNU ls does.
/// Falls back to plain byte comparison if either name contains a null byte.
fn strcoll(a: &str, b: &str) -> std::cmp::Ordering {
    use std::ffi::CString;
    let ca = CString::new(a.as_bytes());
    let cb = CString::new(b.as_bytes());
    match (ca, cb) {
        (Ok(ca), Ok(cb)) => {
            let r = unsafe { libc::strcoll(ca.as_ptr(), cb.as_ptr()) };
            r.cmp(&0)
        }
        _ => a.cmp(b),
    }
}

/// Must be called once at program startup (in main) so that strcoll() sorts
/// filenames the same way GNU ls does — using the user's system locale.
///
///     use executers::ls::init_locale;
///     init_locale();
pub fn init_locale() {
    unsafe {
        // "" tells setlocale to read LC_ALL/LC_COLLATE/LANG from the environment,
        // exactly what GNU ls does at startup.
        libc::setlocale(libc::LC_ALL, c"".as_ptr());
    }
}

/// Major device number decoded from a raw Linux `rdev` value.
fn dev_major(rdev: u64) -> u64 {
    ((rdev >> 8) & 0x000_fff) | ((rdev >> 32) & !0x000_fff)
}

/// Minor device number decoded from a raw Linux `rdev` value.
fn dev_minor(rdev: u64) -> u64 {
    (rdev & 0x0000_00ff) | ((rdev >> 12) & !0x0000_00ff)
}

/// Converts the Unix mode bits into the 9-character rwxrwxrwx permission string.
/// Correctly renders setuid (s/S), setgid (s/S), and sticky (t/T) bits.
fn perms_to_string(mode: u32) -> String {
    let mut s = String::with_capacity(9);

    // owner read / write / execute+setuid
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o100 != 0, mode & 0o4000 != 0) {
        (true,  true)  => 's', // executable + setuid
        (false, true)  => 'S', // setuid but NOT executable
        (true,  false) => 'x',
        (false, false) => '-',
    });

    // group read / write / execute+setgid
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o010 != 0, mode & 0o2000 != 0) {
        (true,  true)  => 's', // executable + setgid
        (false, true)  => 'S', // setgid but NOT executable
        (true,  false) => 'x',
        (false, false) => '-',
    });

    // other read / write / execute+sticky
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o001 != 0, mode & 0o1000 != 0) {
        (true,  true)  => 't', // executable + sticky
        (false, true)  => 'T', // sticky but NOT executable
        (true,  false) => 'x',
        (false, false) => '-',
    });

    s
}

/// Checks for extended attributes via `llistxattr(2)` (lstat semantics —
/// does NOT follow symlinks).  Returns '+' if any xattrs exist, ' ' if not.
///
/// Add to Cargo.toml:  libc = "0.2"
fn acl_indicator(path: &Path) -> char {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let c_path = match CString::new(path.as_os_str().as_bytes()) {
        Ok(p)  => p,
        Err(_) => return ' ',
    };
    // Passing a null buffer makes llistxattr return only the required buffer
    // size.  Any value > 0 means at least one xattr name is present.
    let size = unsafe {
        libc::llistxattr(c_path.as_ptr(), std::ptr::null_mut(), 0)
    };
    if size > 0 { '+' } else { ' ' }
}

/// Formats a Unix timestamp exactly as GNU `ls -l` does:
///   recent file (within ~6 months)  →  "Mon DD HH:MM"
///   old / future file               →  "Mon DD  YYYY"
fn format_time(secs: i64) -> String {
    let t = UNIX_EPOCH
        .checked_add(Duration::from_secs(secs.max(0) as u64))
        .unwrap_or(UNIX_EPOCH);
    let datetime: chrono::DateTime<chrono::Local> = t.into();
    let now        = chrono::Local::now();
    let six_months = chrono::Duration::days(182);

    if datetime < now - six_months || datetime > now + six_months {
        datetime.format("%b %e  %Y").to_string()
    } else {
        datetime.format("%b %e %H:%M").to_string()
    }
}

// ── Grid display (ls without -l) ─────────────────────────────────────────────

impl fmt::Display for LsNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        let mut grid = Grid::new(GridOptions {
            filling:   Filling::Spaces(2),
            direction: Direction::TopToBottom,
        });

        // Only apply shell quoting when stdout is a TTY — same as GNU ls.
        let is_tty = unsafe { libc::isatty(libc::STDOUT_FILENO) } == 1;
        let display_name = |n: &str| -> String {
            if is_tty { shell_quote(n) } else { n.to_string() }
        };

        for name in &self.0 {
            grid.add(Cell::from(display_name(name)));
        }

        let term_width   = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
        let max_name_len = self.0.iter().map(|n| display_name(n).len()).max().unwrap_or(1);
        let num_cols     = ((term_width + 2) / (max_name_len + 2)).max(1);

        write!(f, "{}", grid.fit_into_columns(num_cols))
    }
}

/// Quotes a filename the way GNU ls shell-quoting style does on a TTY:
///   - no special chars  →  as-is
///   - special chars     →  single-quoted, with ' escaped as '\''
///   - non-printable     →  appended as $'\n' after the closing quote
fn shell_quote(name: &str) -> String {
    let needs_quotes = name.chars().any(|c| {
        matches!(c, ' ' | '\t' | '\n' | '\r' | '\'' | '"' | '\\'
                  | '!' | '#' | '$' | '&' | '(' | ')' | '*' | ';'
                  | '<' | '>' | '?' | '[' | ']' | '^' | '`' | '{' | '|' | '}' | '~')
        || (c as u32) < 32 || c == '\x7f'
    });

    if !needs_quotes {
        return name.to_string();
    }

    let mut s      = String::from("'");
    let mut suffix = String::new();

    for c in name.chars() {
        match c {
            // Each single-quote becomes: '\''
            // (close quote, backslash-quote, reopen quote)
            '\'' => s.push_str("'\\''"),
            '\n' => suffix.push_str("$'\\n'"),
            '\r' => suffix.push_str("$'\\r'"),
            '\t' => suffix.push_str("$'\\t'"),
            c if (c as u32) < 32 || c == '\x7f' => {
                suffix.push_str(&format!("$'\\x{:02x}'", c as u32));
            }
            c => s.push(c),
        }
    }

    s.push('\'');
    s.push_str(&suffix);
    s
}