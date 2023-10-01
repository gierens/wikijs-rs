use fuser::MountOption::FSName;
use fuser::{
    mount2, FileAttr, Filesystem, ReplyAttr, ReplyData, ReplyDirectory,
    ReplyEntry, ReplyWrite, Request, TimeOrNow,
};
use libc::{EINVAL, EIO, EISDIR, ENOENT, O_TRUNC};
use wikijs::page::{Page, PageTreeItem, PageTreeMode};
use wikijs::{Api, Credentials};

use chrono::DateTime;
use clap::Parser;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::exit;
use std::time::SystemTime;

#[allow(unused_imports)]
use colored::Colorize;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

mod page;

#[allow(clippy::large_enum_variant)]
enum Inode {
    Page(Page),
    Directory(Vec<PageTreeItem>),
}

fn parse_systemtime(str: String) -> SystemTime {
    match DateTime::parse_from_rfc3339(&str) {
        Ok(dt) => dt.into(),
        Err(_) => {
            warn!("parse_systemtime: failed to parse {}", str);
            SystemTime::now()
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<FileAttr> for Inode {
    fn into(self) -> FileAttr {
        match self {
            Inode::Page(page) => {
                let update_time = parse_systemtime(page.updated_at);
                let create_time = parse_systemtime(page.created_at);
                FileAttr {
                    ino: page.id as u64 | 0x80000000_00000000,
                    size: page.content.len() as u64,
                    blocks: 1,
                    atime: update_time,
                    mtime: update_time,
                    ctime: update_time,
                    crtime: create_time,
                    kind: fuser::FileType::RegularFile,
                    perm: 0o644,
                    nlink: 1,
                    uid: 0,
                    gid: 0,
                    rdev: 0,
                    blksize: 0,
                    flags: 0,
                }
            }
            Inode::Directory(page_tree) => {
                let ino = if !page_tree.is_empty() {
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
    Directory(i64),
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
    locale: String,
}

impl Fs {
    pub fn new(api: Api, locale: String) -> Self {
        Self { api, locale }
    }

    fn get_inode(&self, ino: u64) -> Option<Inode> {
        match InodeType::from(ino) {
            InodeType::Page(id) => {
                debug!("get_inode: page {}", id);
                match self.api.page_get(id) {
                    Ok(page) => Some(Inode::Page(page)),
                    Err(_) => None,
                }
            }
            InodeType::Directory(id) => {
                debug!("get_inode: directory {}", id);
                match self.api.page_tree_get(
                    id,
                    PageTreeMode::ALL,
                    true,
                    self.locale.clone(),
                ) {
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
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
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

    /// Set attributes of an inode.
    ///
    /// # Arguments
    /// * `req` - The request.
    /// * `ino` - The inode number.
    /// * `mode` - The mode of the inode.
    /// * `uid` - The user ID of the inode.
    /// * `gid` - The group ID of the inode.
    /// * `size` - The size of the inode.
    /// * `atime` - The access time of the inode.
    /// * `mtime` - The modification time of the inode.
    /// * `fh` - The file handle.
    /// * `crtime` - The creation time of the inode.
    /// * `chgtime` - The change time of the inode.
    /// * `bkuptime` - The backup time of the inode.
    /// * `flags` - The flags of the inode.
    /// * `reply` - The reply.
    ///
    /// # Returns
    /// Nothing.
    fn setattr(
        &mut self,
        _req: &Request,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<TimeOrNow>,
        mtime: Option<TimeOrNow>,
        _ctime: Option<SystemTime>,
        fh: Option<u64>,
        crtime: Option<SystemTime>,
        chgtime: Option<SystemTime>,
        bkuptime: Option<SystemTime>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        let start = SystemTime::now();
        info!(
            "setattr(ino={}, mode={:?}, uid={:?}, gid={:?}, size={:?}, \
              atime={:?}, mtime={:?}, fh={:?}, crtime={:?}, chgtime={:?}, \
              bkuptime={:?}, flags={:?})",
            ino,
            mode,
            uid,
            gid,
            size,
            atime,
            mtime,
            fh,
            crtime,
            chgtime,
            bkuptime,
            flags
        );

        let inode = match self.get_inode(ino) {
            Some(inode) => inode,
            None => {
                warn!("setattr: inode {} not found", ino);
                reply.error(ENOENT);
                return;
            }
        };

        let page = match inode {
            Inode::Page(page) => page,
            _ => {
                warn!("setattr: inode {} is not a page", ino);
                reply.error(EINVAL);
                return;
            }
        };

        if let Some(size) = size {
            let mut content = page.content.clone();
            if size < content.len() as u64 {
                content.truncate(std::cmp::max(size as usize, 1));
            }
            match self.api.page_update_content(page.id, content) {
                Ok(_) => {
                    debug!("setattr: updated inode {}", ino);
                    let attr = match self.get_inode(ino) {
                        Some(inode) => inode.into(),
                        None => {
                            warn!("setattr: inode {} not found", ino);
                            reply.error(ENOENT);
                            return;
                        }
                    };
                    reply.attr(
                        &SystemTime::now().duration_since(start).unwrap(),
                        &attr,
                    );
                    return;
                }
                Err(_) => {
                    error!("setattr: failed to update inode {}", ino);
                    reply.error(EIO);
                    return;
                }
            }
        }

        let attr = Inode::Page(page).into();
        reply.attr(&SystemTime::now().duration_since(start).unwrap(), &attr);
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
        mut reply: ReplyDirectory,
    ) {
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
            let basename = pti.path.split('/').last().unwrap();
            if pti.is_folder {
                if reply.add(
                    pti.id as u64 + 1,
                    next_offset,
                    fuser::FileType::Directory,
                    basename,
                ) {
                    debug!("readdir: buffer full at offset {}", next_offset);
                    reply.ok();
                    return;
                }
                i += 1;
                next_offset += 1;
            }
            if let Some(pid) = pti.page_id {
                let filename = format!("{}.md", basename);
                if reply.add(
                    pid as u64 | 0x80000000_00000000,
                    next_offset,
                    fuser::FileType::RegularFile,
                    filename,
                ) {
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
        reply: ReplyEntry,
    ) {
        let start = SystemTime::now();
        let mut name_str = name.to_str().unwrap();
        info!("lookup(parent={}, name={:?})", parent, name_str);
        let mut is_dir = true;
        if name_str.ends_with(".md") {
            name_str = &name_str[..name_str.len() - 3];
            is_dir = false;
        }

        let page_tree = match self.get_inode(parent) {
            Some(Inode::Directory(page_tree)) => page_tree,
            _ => {
                warn!("lookup: parent inode {} is not a directory", parent);
                reply.error(ENOENT);
                return;
            }
        };

        for pti in page_tree {
            if pti.path.split('/').last().unwrap() == name_str {
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
        reply: ReplyData,
    ) {
        info!(
            "read(ino={}, fh={}, offset={}, size={}, flags={:?}, \
              lock_owner={:?})",
            ino, fh, offset, size, flags, lock_owner
        );

        if let InodeType::Directory(_) = InodeType::from(ino) {
            warn!("read: inode {} is a directory", ino);
            reply.error(EISDIR);
            return;
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
            warn!(
                "read: invalid offset {} for file of size {} with inode {}",
                offset, size, ino
            );
            reply.error(ENOENT);
            return;
        }

        let end = (offset as u64 + size as u64).min(content_size);
        let data = page.content[offset as usize..end as usize].to_string();
        reply.data(data.as_bytes());
    }

    /// Write data to a file.
    ///
    /// # Arguments
    /// * `req` - The request.
    /// * `ino` - The inode number.
    /// * `fh` - The file handle.
    /// * `offset` - The offset in the file.
    /// * `data` - The data to write.
    /// * `write_flags` - The write flags.
    /// * `flags` - The flags.
    /// * `lock_owner` - The lock owner.
    /// * `reply` - The reply.
    ///
    /// # Returns
    /// Nothing.
    fn write(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        write_flags: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        info!(
            "write(ino={}, fh={}, offset={}, data={:?}, write_flags={:?}, \
              flags={:?}, lock_owner={:?})",
            ino, fh, offset, data, write_flags, flags, lock_owner
        );

        if let InodeType::Directory(_) = InodeType::from(ino) {
            warn!("write: inode {} is a directory", ino);
            reply.error(EISDIR);
            return;
        }

        let mut page = match self.get_inode(ino) {
            Some(Inode::Page(page)) => page,
            _ => {
                warn!("write: inode {} not found", ino);
                reply.error(ENOENT);
                return;
            }
        };

        let size = page.content.len() as u64;

        if offset < 0 || offset as u64 > size {
            warn!(
                "write: invalid offset {} for file of size {} with inode {}",
                offset, size, ino
            );
            reply.error(EINVAL);
            return;
        }

        let end = offset as usize + data.len();
        if end < page.content.len() && write_flags as i32 == O_TRUNC {
            debug!(
                "write: truncating inode {} from {} to {}",
                ino,
                page.content.len(),
                end
            );
            page.content.truncate(end);
        }

        // TODO maybe page content should be mutable, or all fields actually
        // merge contents
        let mut content = page.content[..offset as usize].to_string()
            + &String::from_utf8_lossy(data);
        if end < page.content.len() {
            content += &page.content[end..];
        }
        debug!("write: inode {} from {} to {}", ino, offset, end);

        match self.api.page_update_content(page.id, content) {
            Ok(_) => {
                debug!("write: updated inode {}", ino);
                reply.written(data.len() as u32);
            }
            Err(_) => {
                error!("write: failed to update inode {}", ino);
                reply.error(EIO);
            }
        }
    }

    /// Open a file.
    ///
    /// # Arguments
    /// * `req` - The request.
    /// * `ino` - The inode number.
    /// * `flags` - The flags of the file.
    /// * `reply` - The reply.
    ///
    /// # Returns
    /// Nothing.
    // fn open(
    //     &mut self,
    //     _req: &Request<'_>,
    //     ino: u64,
    //     flags: i32,
    //     reply: ReplyOpen
    // ) {
    //     info!("open(ino={}, flags={:?})", ino, flags);
    //     reply.opened(0, flags.try_into().unwrap());
    // }

    /// Create a file node.
    ///
    /// # Arguments
    /// * `req` - The request.
    /// * `parent` - The parent inode number.
    /// * `name` - The name of the file.
    /// * `mode` - The mode of the file.
    /// * `umask` - The umask of the file.
    /// * `flags` - The flags of the file.
    /// * `reply` - The reply.
    ///
    /// # Returns
    /// Nothing.
    fn mknod(
        &mut self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        rdev: u32,
        reply: ReplyEntry,
    ) {
        let _start = SystemTime::now();
        info!(
            "mknod(parent={}, name={:?}, mode={}, umask={}, rdev={})",
            parent, name, mode, umask, rdev
        );
        reply.error(EINVAL);
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

    #[clap(
        short,
        long,
        default_value = "en",
        help = "Wiki.js locale to use",
        env = "WIKI_JS_LOCALE"
    )]
    locale: String,

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
    // set_max_level(cli.verbose.log_level_filter());

    // env_logger::builder()
    //     .format_timestamp(None)
    //     .format_module_path(false)
    //     .filter_level(cli.verbose.log_level_filter())
    //     .init();

    if !cli.mountpoint.exists() || !cli.mountpoint.is_dir() {
        error!(
            "Mountpoint {} does not exist or is not a directory",
            cli.mountpoint.display()
        );
        exit(1);
    }

    let credentials = Credentials::Key(cli.key);
    let api = Api::new(cli.url, credentials);
    let fs = Fs::new(api, cli.locale);

    mount2(fs, &cli.mountpoint, &[FSName("wikijs-fuse".to_string())])
        .unwrap_or_else(|error| {
            error!("{}", error);
            exit(1);
        });
}
