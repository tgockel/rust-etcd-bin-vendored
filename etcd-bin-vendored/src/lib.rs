#![doc = include_str!("../README.md")]

use std::{env, error, fmt, path::Path};

/// Error returned when `etcd` is not supported on the requested OS and architecture.
pub struct ArchitectureNotSupported {
    inner: ArchitectureNotSupportedInner,
}

enum ArchitectureNotSupportedInner {
    Unknown {
        os: &'static str,
        arch: &'static str,
    },
    #[allow(dead_code, reason = "this is only used when a feature is disabled")]
    Disabled(Crate),
}

impl fmt::Display for ArchitectureNotSupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ArchitectureNotSupportedInner::Unknown { os, arch } => write!(f, "etcd not supported for {os}-{arch}"),
            ArchitectureNotSupportedInner::Disabled(feature_name) => {
                write!(f, "etcd supported, but the {feature_name} feature is not enabled")
            }
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
            ArchitectureNotSupportedInner::Disabled(crate_name) => f
                .debug_tuple("ArchitectureNotSupported::Disabled")
                .field(&crate_name)
                .finish(),
        }
    }
}

impl error::Error for ArchitectureNotSupported {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Crate {
    LinuxAMD64,
    LinuxARM64,
    LinuxPPC64LE,
    LinuxS390x,
    MacAMD64,
    MacARM64,
    WindowsAMD64,
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LinuxAMD64 => write!(f, "linux-amd64"),
            Self::LinuxARM64 => write!(f, "linux-arm64"),
            Self::LinuxPPC64LE => write!(f, "linux-ppc64le"),
            Self::LinuxS390x => write!(f, "linux-s390x"),
            Self::MacAMD64 => write!(f, "macos-amd64"),
            Self::MacARM64 => write!(f, "macos-arm64"),
            Self::WindowsAMD64 => write!(f, "windows-amd64"),
        }
    }
}

impl Crate {
    pub fn detect() -> Result<Self, ArchitectureNotSupported> {
        match (env::consts::OS, env::consts::ARCH) {
            ("linux", "x86_64") => Ok(Self::LinuxAMD64),
            ("linux", "aarch64") => Ok(Self::LinuxARM64),
            ("linux", "powerpc64") => Ok(Self::LinuxPPC64LE),
            ("linux", "s390x") => Ok(Self::LinuxS390x),
            ("macos", "x86_64") => Ok(Self::MacAMD64),
            ("macos", "aarch64") => Ok(Self::MacARM64),
            ("windows", "x86_64") => Ok(Self::WindowsAMD64),
            (os, arch) => Err(ArchitectureNotSupported {
                inner: ArchitectureNotSupportedInner::Unknown { os, arch },
            }),
        }
    }
}

macro_rules! match_platform {
    ($detected:expr; $( ($variant:ident, $os:literal, $arch:literal, $feature:literal, $module:ident) ),* $(,)?) => {
        match $detected {
            $(
                #[cfg(any(all(target_os = $os, target_arch = $arch), feature = $feature))]
                Crate::$variant => $module::etcd_bin_path().map_err(|_| unreachable!()),
                #[cfg(not(any(all(target_os = $os, target_arch = $arch), feature = $feature)))]
                Crate::$variant => Err(ArchitectureNotSupported {
                    inner: ArchitectureNotSupportedInner::Disabled($detected)
                }),
            )*
        }
    };
}

/// Return a path to the `etcd` binary for this platform.
///
/// This function returns the path to the `etcd` program when it is supported on the current platform. In the case of an
/// unsupported platform, an `Err(ArchitectureNotSupported)` is returned with a description of the error.
pub fn etcd_bin_path() -> Result<&'static Path, ArchitectureNotSupported> {
    let detected = Crate::detect()?;

    match_platform!(detected;
        (LinuxAMD64,    "linux",    "x86_64",    "linux-amd64",    etcd_bin_vendored_linux_amd64),
        (LinuxARM64,    "linux",    "aarch64",   "linux-arm64",    etcd_bin_vendored_linux_arm64),
        (LinuxPPC64LE,  "linux",    "powerpc64", "linux-ppc64le",  etcd_bin_vendored_linux_ppc64le),
        (LinuxS390x,    "linux",    "s390x",     "linux-s390x",    etcd_bin_vendored_linux_s390x),
        (MacAMD64,      "macos",    "x86_64",    "macos-amd64",    etcd_bin_vendored_darwin_amd64),
        (MacARM64,      "macos",    "aarch64",   "macos-arm64",    etcd_bin_vendored_darwin_arm64),
        (WindowsAMD64,  "windows",  "x86_64",    "windows-amd64",  etcd_bin_vendored_windows_amd64),
    )
}
