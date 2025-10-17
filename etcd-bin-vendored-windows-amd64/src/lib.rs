#![doc = include_str!("../README.md")]

use std::{convert::Infallible, path::Path};

/// See [`etcd_bin_vendored::etcd_bin_path`][etcd_bin_path].
///
/// [etcd_bin_path]: https://docs.rs/etcd-bin-vendored/latest/etcd_bin_vendored/fn.etcd_bin_path.html
pub fn etcd_bin_path() -> Result<&'static Path, Infallible> {
    Ok(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/etcd.exe")))
}

#[cfg(test)]
mod tests {
    #[test]
    fn file_exists() {
        assert!(crate::etcd_bin_path().unwrap().exists())
    }
}
