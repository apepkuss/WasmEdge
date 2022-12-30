use crate::file::FileCaps;
use bitflags::bitflags;

pub trait WasiDir: Send + Sync {}

pub(crate) struct DirEntry {
    caps: DirCaps,
    file_caps: FileCaps,
    preopen_dir: Option<std::path::PathBuf>,
    dir: Box<dyn WasiDir>,
}
impl DirEntry {
    pub(crate) fn new(
        caps: DirCaps,
        file_caps: FileCaps,
        preopen_dir: Option<std::path::PathBuf>,
        dir: Box<dyn WasiDir>,
    ) -> Self {
        Self {
            caps,
            file_caps,
            preopen_dir,
            dir,
        }
    }
}

bitflags! {
    pub struct DirCaps: u32 {
        const CREATE_DIRECTORY        = 0b1;
        const CREATE_FILE             = 0b10;
        const LINK_SOURCE             = 0b100;
        const LINK_TARGET             = 0b1000;
        const OPEN                    = 0b10000;
        const READDIR                 = 0b100000;
        const READLINK                = 0b1000000;
        const RENAME_SOURCE           = 0b10000000;
        const RENAME_TARGET           = 0b100000000;
        const SYMLINK                 = 0b1000000000;
        const REMOVE_DIRECTORY        = 0b10000000000;
        const UNLINK_FILE             = 0b100000000000;
        const PATH_FILESTAT_GET       = 0b1000000000000;
        const PATH_FILESTAT_SET_TIMES = 0b10000000000000;
        const FILESTAT_GET            = 0b100000000000000;
        const FILESTAT_SET_TIMES      = 0b1000000000000000;
    }
}
