use wikijs::page::{Page, PageTreeItem};
use wikijs::{Api, Credentials};
use fuser::{Filesystem, mount2, Request, ReplyAttr, ReplyDirectory, FileAttr,
            ReplyEntry, ReplyData};
use fuser::MountOption::FSName;
use libc::{ENOENT, EISDIR};

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::exit;
use std::time::SystemTime;
use clap::Parser;

#[allow(unused_imports)]
use colored::Colorize;
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};


enum Inode {
    Page(Page),
    Directory(Vec<PageTreeItem>),
}


impl Into<FileAttr> for Inode {
    fn into(self) -> FileAttr {
        match self {
            Inode::Page(page) => {
                FileAttr {
                    ino: page.id as u64 | 0x80000000_00000000,
                    size: page.content.len() as u64,
                    blocks: 1,
                    atime: SystemTime::now(),
                    mtime: SystemTime::now(),
                    ctime: SystemTime::now(),
                    crtime: SystemTime::now(),
                    kind: fuser::FileType::RegularFile,
                    perm: 0o644,
                    nlink: 1,
                    uid: 0,
                    gid: 0,
                    rdev: 0,
                    blksize: 0,
                    flags: 0,
                }
            },
            Inode::Directory(page_tree) => {
                let ino = if page_tree.len() > 0 {
                    if let Some(id) = page_tree[0].parent {
                        id as u64 + 1
                    } else {
                        0
                    }
                } else {
                    0
                };
                FileAttr {
                    ino,
                    size: 0,
                    blocks: 0,
                    atime: SystemTime::now(),
                    mtime: SystemTime::now(),
                    ctime: SystemTime::now(),
                    crtime: SystemTime::now(),
                    kind: fuser::FileType::Directory,
                    perm: 0o755,
                    nlink: 1,
                    uid: 0,
                    gid: 0,
                    rdev: 0,
                    blksize: 0,
                    flags: 0,
                }
            }
        }
    }
}


enum InodeType {
    Page(i64),
    Directory(i64)
}


impl From<u64> for InodeType {
    fn from(ino: u64) -> Self {
        if ino & 0x80000000_00000000 == 0x80000000_00000000 {
            InodeType::Page((ino & 0x7FFF_FFFF_FFFF_FFFF) as i64)
        } else {
            InodeType::Directory((ino - 1) as i64)
        }
    }
}


struct Fs {
    api: Api,
}


impl Fs {
    pub fn new(api: Api) -> Self {
        Self { api }
    }

    fn get_inode(&self, ino: u64) -> Option<Inode> {
        match InodeType::from(ino) {
            InodeType::Page(id) => {
                debug!("get_inode: page {}", id);
                match self.api.get_page(id) {
                    Ok(page) => Some(Inode::Page(page)),
                    Err(_) => None,
                }
            },
            InodeType::Directory(id) => {
                debug!("get_inode: directory {}", id);
                match self.api.get_page_tree(id) {
                    Ok(page_tree) => Some(Inode::Directory(page_tree)),
                    Err(_) => None,
                }
            }
        }
    }
}


impl Filesystem for Fs {
    /// Get attributes of an inode.
    ///
    /// # Arguments
    /// * `req` - The request.
    /// * `ino` - The inode number.
    /// * `reply` - The reply.
    ///
    /// # Returns
    /// Nothing.
    fn getattr(
        &mut self,
        _req: &Request,
        ino: u64,
        reply: ReplyAttr
    ) {
        let start = SystemTime::now();
        info!("getattr(ino={})", ino);

        let attr = match self.get_inode(ino) {
            Some(inode) => inode.into(),
            None => {
                warn!("getattr: inode {} not found", ino);
                reply.error(ENOENT);
                return;
            }
        };

        let ttl = SystemTime::now().duration_since(start).unwrap();
        reply.attr(&ttl, &attr);
    }

    /// Read entries of a directory.
    ///
    /// # Arguments
    /// * `req` - The request.
    /// * `ino` - The inode number.
    /// * `fh` - The file handle.
    /// * `offset` - The offset in the directory.
    /// * `reply` - The reply.
    ///
    /// # Returns
    /// Nothing.
    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: ReplyDirectory)
    {
        info!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        let mut next_offset = offset + 1;

        // get page tree
        let page_tree = match self.get_inode(ino) {
            Some(Inode::Directory(page_tree)) => page_tree,
            _ => {
                warn!("readdir: inode {} is not a directory", ino);
                reply.error(ENOENT);
                return;
            }
        };

        // add current directory entry
        if offset == 0 {
            if reply.add(ino, 1, fuser::FileType::Directory, ".") {
                debug!("readdir: buffer full at offset 0");
                reply.ok();
                return;
            }
            next_offset += 1;
        }

        // add parent directory entry
        // if offset <= 1 {
        // TODO
        // }

        // add child entries
        let mut i = 0;
        for pti in page_tree {
            if i + 2 <= offset as usize {
                continue;
            }
            let basename = pti.path.split("/").last().unwrap();
            if pti.is_folder {
                if reply.add(pti.id as u64 + 1, next_offset, fuser::FileType::Directory, basename) {
                    debug!("readdir: buffer full at offset {}", next_offset);
                    reply.ok();
                    return;
                }
                i += 1;
                next_offset += 1;
            }
            if let Some(pid) = pti.page_id {
                let filename = format!("{}.md", basename);
                if reply.add(pid as u64 | 0x80000000_00000000, next_offset, fuser::FileType::RegularFile, filename) {
                    debug!("readdir: buffer full at offset {}", next_offset);
                    reply.ok();
                    return;
                }
                i += 1;
                next_offset += 1;
            }
        }

        reply.ok();
    }

    // Lookup inode by name and parent inode.
    //
    // # Arguments
    // * `req` - The request.
    // * `parent` - The parent inode number.
    // * `name` - The name of the inode.
    // * `reply` - The reply.
    //
    // # Returns
    // Nothing.
    fn lookup(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        reply: ReplyEntry
    ) {
        let start = SystemTime::now();
        let mut name_str = name.to_str().unwrap();
        let mut is_dir = true;
        if name_str.ends_with(".md") {
            name_str = &name_str[..name_str.len() - 3];
            is_dir = false;
        }
        info!("lookup(parent={}, name={:?})", parent, name_str);
        
        let page_tree = match self.get_inode(parent) {
            Some(Inode::Directory(page_tree)) => page_tree,
            _ => {
                warn!("lookup: parent inode {} is not a directory", parent);
                reply.error(ENOENT);
                return;
            }
        };

        for pti in page_tree {
            if pti.path.split("/").last().unwrap() == name_str {
                let ino = if is_dir {
                    pti.id as u64 + 1
                } else {
                    pti.page_id.unwrap() as u64 | 0x80000000_00000000
                };
                debug!("lookup: found inode {}", ino);
                let attr = match self.get_inode(ino) {
                    Some(inode) => inode.into(),
                    None => {
                        warn!("lookup: inode {} not found", ino);
                        reply.error(ENOENT);
                        return;
                    }
                };
                let ttl = SystemTime::now().duration_since(start).unwrap();
                reply.entry(&ttl, &attr, 0);
                return;
            }    
        }
    
        warn!("lookup: inode not found");
        reply.error(ENOENT);
    }

    /// Read data from a file.
    ///
    /// # Arguments
    /// * `req` - The request.
    /// * `ino` - The inode number.
    /// * `fh` - The file handle.
    /// * `offset` - The offset in the file.
    /// * `size` - The size of the data to read.
    /// * `flags` - The flags.
    /// * `lock_owner` - The lock owner.
    /// * `reply` - The reply.
    ///
    /// # Returns
    /// Nothing.
    fn read(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyData
    ) {
        info!("read(ino={}, fh={}, offset={}, size={}, flags={:?}, \
              lock_owner={:?})", ino, fh, offset, size, flags, lock_owner);

        match InodeType::from(ino) {
            InodeType::Directory(_) => {
                warn!("read: inode {} is a directory", ino);
                reply.error(EISDIR);
                return;
            }
            _ => {}
        }

        let page = match self.get_inode(ino) {
            Some(Inode::Page(page)) => page,
            _ => {
                warn!("read: inode {} not found", ino);
                reply.error(ENOENT);
                return;
            }
        };

        let content_size = page.content.len() as u64;

        if offset < 0 || offset as u64 > content_size {
            warn!("read: invalid offset {} for file of size {} with inode {}",
                offset, size, ino);
            reply.error(ENOENT);
            return;
        }

        let end = (offset as u64 + size as u64).min(content_size);
        let data = page.content[offset as usize..end as usize].to_string();
        reply.data(&data.as_bytes());
    }
}


#[derive(Parser)]
#[command(name = "wikijs-fuse")]
#[command(author = "Sandro-Alessio Gierens <sandro@gierens.de>")]
#[command(version = "0.1.0")]
#[command(about = "Mount a Wiki.js instance as a FUSE filesystem")]
struct Cli {
    #[clap(short, long, help = "Wiki.js base URL", env = "WIKI_JS_BASE_URL")]
    url: String,

    #[clap(short, long, help = "Wiki.js API key", env = "WIKI_JS_API_KEY")]
    key: String,

    #[clap(help = "Mountpoint", env = "WIKI_JS_FUSE_MOUNTPOINT")]
    mountpoint: PathBuf,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}


fn main() {
    let cli = Cli::parse();
    stderrlog::new()
        .module(module_path!())
        .verbosity(cli.verbose.log_level_filter())
        .init()
        .unwrap();

    if !cli.mountpoint.exists() || !cli.mountpoint.is_dir() {
        error!("Mountpoint {} does not exist or is not a directory",
            cli.mountpoint.display());
        exit(1);
    }

    let credentials = Credentials::Key(cli.key);
    let api = Api::new(cli.url, credentials);
    let fs = Fs::new(api);

    mount2(fs, &cli.mountpoint, &[FSName("wikijs-fuse".to_string()),]
    ).unwrap_or_else(|error| {
        error!("{}", error);
        exit(1);
    });
}
