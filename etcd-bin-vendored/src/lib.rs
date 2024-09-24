#![doc = include_str!("../README.md")]

use std::{env, error, fmt, path::Path};

/// Error returned when `etcd` is not supported on the requested OS and architecture.
#[derive(Debug)]
pub struct ArchitectureNotSupported {
    os: &'static str,
    arch: &'static str,
}

impl fmt::Display for ArchitectureNotSupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "etcd not supported for {}-{}", self.os, self.arch)
    }
}

impl error::Error for ArchitectureNotSupported {}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Crate {
    LinuxAMD64,
    LinuxARM64,
    LinuxPPC64LE,
    //TODO(3.5): LinuxS390x,
    MacAMD64,
    //TODO(3.5): MacARM64,
    WindowsAMD64,
}

impl Crate {
    pub fn detect() -> Result<Self, ArchitectureNotSupported> {
        match (env::consts::OS, env::consts::ARCH) {
            ("linux", "x86_64") => Ok(Self::LinuxAMD64),
            ("linux", "aarch64") => Ok(Self::LinuxARM64),
            ("linux", "powerpc64") => Ok(Self::LinuxPPC64LE),
            //TODO(3.5): ("linux", "s390x") => Ok(Self::LinuxS390x),
            ("macos", "x86_64") => Ok(Self::MacAMD64),
            //TODO(3.5): ("macos", "aarch64") => Ok(Self::MacARM64),
            ("windows", "x86_64") => Ok(Self::WindowsAMD64),
            (os, arch) => Err(ArchitectureNotSupported { os, arch }),
        }
    }
}

/// Return a path to the `etcd` binary for this platform.
///
/// This function returns the path to the `etcd` program when it is supported on the current platform. In the case of an
/// unsupported platform, an `Err(ArchitectureNotSupported)` is returned with a description of the error.
pub fn etcd_bin_path() -> Result<&'static Path, ArchitectureNotSupported> {
    let found = match Crate::detect()? {
        Crate::LinuxAMD64 => etcd_bin_vendored_linux_amd64::etcd_bin_path(),
        Crate::LinuxARM64 => etcd_bin_vendored_linux_arm64::etcd_bin_path(),
        Crate::LinuxPPC64LE => etcd_bin_vendored_linux_ppc64le::etcd_bin_path(),
        //TODO(3.5): Crate::LinuxS390x => etcd_bin_vendored_linux_s390x::etcd_bin_path(),
        Crate::MacAMD64 => etcd_bin_vendored_darwin_amd64::etcd_bin_path(),
        //TODO(3.5): Crate::MacARM64 => etcd_bin_vendored_darwin_arm64::etcd_bin_path(),
        Crate::WindowsAMD64 => etcd_bin_vendored_windows_amd64::etcd_bin_path(),
    };

    Ok(found.unwrap())
}
