#![doc = include_str!("../README.md")]

use std::{error, fmt, path::Path};

/// Error returned when `etcd` is not supported on the requested OS and architecture.
pub struct ArchitectureNotSupported {
    inner: ArchitectureNotSupportedInner,
}

enum ArchitectureNotSupportedInner {
    Unknown { os: &'static str, arch: &'static str },
}

impl fmt::Display for ArchitectureNotSupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ArchitectureNotSupportedInner::Unknown { os, arch } => write!(f, "etcd not supported for {os}-{arch}"),
        }
    }
}

impl fmt::Debug for ArchitectureNotSupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ArchitectureNotSupportedInner::Unknown { os, arch } => f
                .debug_struct("ArchitectureNotSupported::Unknown")
                .field("os", &os)
                .field("arch", &arch)
                .finish(),
        }
    }
}

impl error::Error for ArchitectureNotSupported {}

macro_rules! match_platform {
    ($(($os:literal, $arch:literal, $module:ident)),* $(,)?) => {
        match (::std::env::consts::OS, ::std::env::consts::ARCH) {
            $(
                #[cfg(all(target_os = $os, target_arch = $arch))]
                ($os, $arch) => $module::etcd_bin_path().map_err(|_| unreachable!()),
            )*
            (os, arch) => Err(ArchitectureNotSupported {
                inner: ArchitectureNotSupportedInner::Unknown { os, arch }
            })
        }
    }
}

/// Return a path to the `etcd` binary for this platform.
///
/// This function returns the path to the `etcd` program when it is supported on the current platform. In the case of an
/// unsupported platform, an `Err(ArchitectureNotSupported)` is returned with a description of the error.
pub fn etcd_bin_path() -> Result<&'static Path, ArchitectureNotSupported> {
    match_platform! {
        ("linux",    "x86_64",    etcd_bin_vendored_linux_amd64),
        ("linux",    "aarch64",   etcd_bin_vendored_linux_arm64),
        ("linux",    "powerpc64", etcd_bin_vendored_linux_ppc64le),
        ("linux",    "s390x",     etcd_bin_vendored_linux_s390x),
        ("macos",    "x86_64",    etcd_bin_vendored_darwin_amd64),
        ("macos",    "aarch64",   etcd_bin_vendored_darwin_arm64),
        ("windows",  "x86_64",    etcd_bin_vendored_windows_amd64),
        ("windows",  "aarch64",   etcd_bin_vendored_windows_amd64),
    }
}

#[cfg(test)]
mod tests {
    use super::etcd_bin_path;

    #[test]
    fn path_exists() {
        etcd_bin_path().expect("this should work on the platform");
    }
}
