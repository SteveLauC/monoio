use std::{ffi::CString, path::Path};

use super::{Op, OpAble};
use crate::driver::util::cstr;

pub(crate) struct Rename {
    from: CString,
    to: CString,
}

impl Op<Rename> {
    pub(crate) fn rename(from: &Path, to: &Path) -> std::io::Result<Self> {
        let from = cstr(from)?;
        let to = cstr(to)?;

        Op::submit_with(Rename { from, to })
    }
}

impl OpAble for Rename {
    #[cfg(all(target_os = "linux", feature = "iouring"))]
    fn uring_op(&mut self) -> io_uring::squeue::Entry {
        use io_uring::{opcode::RenameAt, types};
        use libc::AT_FDCWD;

        RenameAt::new(
            types::Fd(AT_FDCWD),
            self.from.as_ptr(),
            types::Fd(AT_FDCWD),
            self.to.as_ptr(),
        )
        .build()
    }

    fn legacy_interest(&self) -> Option<(crate::driver::ready::Direction, usize)> {
        None
    }

    #[cfg(all(any(feature = "legacy", feature = "poll-io"), unix))]
    fn legacy_call(&mut self) -> std::io::Result<u32> {
        use crate::syscall_u32;

        syscall_u32!(renameat(
            libc::AT_FDCWD,
            self.from.as_ptr(),
            libc::AT_FDCWD,
            self.to.as_ptr()
        ))
    }

    #[cfg(all(any(feature = "legacy", feature = "poll-io"), windows))]
    fn legacy_call(&mut self) -> io::Result<u32> {
        unimplemented!()
    }
}