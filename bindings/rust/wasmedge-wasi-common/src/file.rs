use bitflags::bitflags;

bitflags! {
    pub struct FileCaps : u32 {
        const DATASYNC           = 0b1;
        const READ               = 0b10;
        const SEEK               = 0b100;
        const FDSTAT_SET_FLAGS   = 0b1000;
        const SYNC               = 0b10000;
        const TELL               = 0b100000;
        const WRITE              = 0b1000000;
        const ADVISE             = 0b10000000;
        const ALLOCATE           = 0b100000000;
        const FILESTAT_GET       = 0b1000000000;
        const FILESTAT_SET_SIZE  = 0b10000000000;
        const FILESTAT_SET_TIMES = 0b100000000000;
        const POLL_READWRITE     = 0b1000000000000;
    }
}
