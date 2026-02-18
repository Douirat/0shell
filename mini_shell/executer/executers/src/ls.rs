use types::command::*;
use types::state::*;
use std::fs::DirEntry;
// use std::ffi::OsString;
use std::fmt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::*;
use std::time::{UNIX_EPOCH, Duration};
use users::{get_user_by_uid, get_group_by_gid};
use std::fs::read_dir;

// file name representer:
#[derive(Debug, Clone)]
struct LsNames(Vec<String>);

// create a structure to represent the file:
#[derive(Debug, Clone, Default)]
pub struct FileEntry {
    pub permissions: String,
    pub sign: char,
    pub links: u64,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub blocks: u64,
    pub mtime: i64,
    pub name: String,
}

// the file entry displayer.
impl fmt::Display for FileEntry{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let user = get_user_by_uid(self.uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or(self.uid.to_string());

        let group = get_group_by_gid(self.gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or(self.gid.to_string());
        write!(
            f,
            "{} {:>2} {:>5} {:>5} {:>8} {} {}",
            self.permissions,
            self.links,
            user,
            group,
            self.size,
            format_time(self.mtime),
            self.name,
        )
    }
}


impl FileEntry{

pub fn new()->  FileEntry {
    FileEntry::default()
}

pub fn get_entry_from_entry(&mut self, entry :DirEntry ) -> Result<Self, std::io::Error>{
    self.name = entry.file_name().to_str().unwrap_or("<invalid utf8>").to_string();
    let meta = entry.metadata()?;
    //   println!("{:?}", meta);
    let ft = &meta.file_type();
    let file_type = match (ft.is_file(), ft.is_dir(), ft.is_symlink()){
    (true, false, false) => '-',
    (false, true, false) => 'd',
    (false, false, true) => 'l',
    (_, _, _) => unreachable!(),
};

let file_sign = match (ft.is_file(), ft.is_dir(), ft.is_symlink()) {
    (true, false, false) => '*',
    (false, true, false) => '/',
    (false, false, true) => '@',
    _ => unreachable!(),
};

self.sign = file_sign;

let permissions = meta.permissions().mode();
let perm_string = perms_to_string(permissions);
self.permissions = file_type.to_string() + &perm_string;

self.links = meta.nlink();

self.uid = meta.uid();
self.gid = meta.gid();

self.size = meta.len();
self.blocks = meta.blocks();

self.mtime = meta.mtime();
Ok(self.clone())
}

// file entry from entry:
pub fn get_entry_from_path(&mut self, path: &PathBuf)-> Result<&mut FileEntry, std::io::Error> {
   
    let meta = path.metadata()?;

    let permissions = perms_to_string(meta.mode()); // your helper
    let links = meta.nlink();
    let uid = meta.uid();
    let gid = meta.gid();
    let size = meta.len();
    let mtime = meta.mtime(); // or mtime() + nsec for full precision

    self.permissions = permissions;
    self.links = links;
    self.uid = uid;
    self.gid = gid;
    self.size = size;
    self.blocks = meta.blocks();
    self.mtime = mtime;

     let ft = &meta.file_type();
    let file_sign = match (ft.is_file(), ft.is_dir(), ft.is_symlink()) {
    (true, false, false) => '*',
    (false, true, false) => '/',
    (false, false, true) => '@',
    _ => unreachable!(),
};

self.sign = file_sign;

Ok(self)
}

}


pub fn ls<'a>(command: &Command) {
// let marker = 0;
if command.args.is_empty() {
        match list(&command.state.clone(), &command.flags, ".".to_string()){
            Ok(()) => {},
            Err(_) => println!("No such file or directory"),
        };
} else {
    for arg in &command.args{
        match  list(&command.state.clone(), &command.flags, arg.clone()){
            Ok(()) => {},
            Err(_) => println!("No such file or directory: {arg}"),
        }
    }
}
}

pub fn list<'a>(state: &'a State,flags: &Vec<Flag>,  arg: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_entries:Vec<FileEntry> = Vec::new();

       // Get the current file (.):
       let current = state.cwd.borrow().clone();
       let mut current_file = FileEntry::new();
       current_file.get_entry_from_path(&current)?;
       current_file.name = String::from(".");
       file_entries.push(current_file);
       
       if *state.cwd.borrow() != Path::new("/") {
    // Get the parent file (..):
        let parent =  state.cwd.borrow().parent().unwrap().to_path_buf();
       let mut parent_file = FileEntry::new();
       parent_file.get_entry_from_path(&parent)?;
       parent_file.name = String::from("..");
       file_entries.push(parent_file);
       }
   
   
    let target = state.cwd.borrow().join(arg).canonicalize()?;


// get the metadata of each directory:
     if let Ok(entries) = read_dir(&target){
        for entry in entries {
         if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                let mut f = FileEntry::new();
                f.get_entry_from_entry(entry)?;
                file_entries.push(f);
            }
        }
    }
    // sort the file entries:
   file_entries.sort_by(|a, b| {
    fn sort_key(name: &str) -> (char, String) {
        let mut chars = name.chars();

        match (chars.next(), chars.next()) {
            (Some('.'), Some(second)) if second.is_alphabetic() => {
                (second.to_ascii_lowercase(), name.to_ascii_lowercase())
            }
            _ => {
                (name.chars().next().unwrap_or('\0').to_ascii_lowercase(),
                 name.to_ascii_lowercase())
            }
        }
    }

    sort_key(&a.name).cmp(&sort_key(&b.name))
});


    // remove hidden files whne the flag -a is abcent:
    if !flags.contains(&Flag::A){
        file_entries.retain(|f| {
            !f.name.starts_with('.')
        });
    }

    // add the file type when the -F is present:
    if flags.contains(&Flag::F){
        file_entries = file_entries
        .into_iter()
        .map(|mut f| {
            f.name.push(f.sign.clone());
            f
        })
        .collect();
    }

    // if the command contains the '-l' flag. 
    if flags.contains(&Flag::L) {
        let total_blocks: u64 = file_entries.iter()
            .filter(|f| f.name != "." && f.name != "..")
            .map(|f| f.blocks)
            .sum::<u64>() / 2;
        println!("total {}", total_blocks);

        // Display all entries:
        for file in file_entries{
            println!("{}", file);
        }
    } else {
        let names:Vec<_> = file_entries.iter().map(|f| f.name.clone()).collect();
        let file_names = LsNames(names);
        println!("{}", file_names);
    }
    

    Ok(())
}


// to comprehend.
fn perms_to_string(mode: u32) -> String {
    let mut s = String::new();

    let flags = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];

    for (bit, ch) in flags {
        if mode & bit != 0 {
            s.push(ch);
        } else {
            s.push('-');
        }
    }

    s
}

fn format_time(secs: i64) -> String {
    let t = UNIX_EPOCH + Duration::from_secs(secs as u64);
    let datetime: chrono::DateTime<chrono::Local> = t.into();
    datetime.format("%b %e %H:%M").to_string()
}

// the normal displayer for file names without flags:


impl fmt::Display for LsNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, name) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{name}")?;
        }
        Ok(())
    }
}