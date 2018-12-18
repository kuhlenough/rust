// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::process_common::{Command, ExitStatus, ExitCode, Stdio, StdioPipes};
pub use self::process_inner::Process;

mod process_common;
#[cfg(not(any(target_os = "fuchsia", target_os = "vxworks")))]
#[path = "process_unix.rs"]
mod process_inner;
#[cfg(target_os = "fuchsia")]
#[path = "process_fuchsia.rs"]
mod process_inner;
#[cfg(target_os = "vxworks")]
#[path = "process_vxworks.rs"]
mod process_inner;
#[cfg(target_os = "fuchsia")]
mod zircon;
#[cfg(target_os = "vxworks")]
mod rtp;
