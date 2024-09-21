use std::{convert::Infallible, path::Path};

/// See [`etcd_bin_vendored::etcd_bin_path`].
pub fn etcd_bin_path() -> Result<&'static Path, Infallible> {
    Ok(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/etcd")))
}

#[cfg(test)]
mod tests {
    #[test]
    fn file_exists() {
        assert!(crate::etcd_bin_path().unwrap().exists())
    }
}
