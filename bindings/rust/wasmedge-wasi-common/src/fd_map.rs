use crate::error::WasiCommonError;
use std::any::Any;
use std::collections::HashMap;

pub struct FdMap {
    map: HashMap<i32, Box<dyn Any + Send + Sync>>,
    next_fd: i32,
}
impl FdMap {
    /// Creates an empty `FdMap`. New insertions will start from `3`, as `0`, `1`, and `2` are reserved for stdin, stdout, and stderr.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_fd: 3, // 0, 1, 2 are reserved for stdin, stdout, stderr
        }
    }

    /// Inserts a new resource at a certain index.
    pub fn insert_at(&mut self, fd: i32, value: Box<dyn Any + Send + Sync>) {
        self.map.insert(fd, value);
    }

    /// Inserts a new resource at the next available index.
    pub fn push(&mut self, value: Box<dyn Any + Send + Sync>) -> Result<i32, WasiCommonError> {
        if self.map.len() == i32::MAX as usize {
            return Err(WasiCommonError::FdMapFull);
        }
        loop {
            let new_fd = self.next_fd;
            self.next_fd = self.next_fd.wrapping_add(1);
            if self.map.contains_key(&new_fd) {
                continue;
            }
            self.map.insert(new_fd, value);
            return Ok(new_fd);
        }
    }
}
