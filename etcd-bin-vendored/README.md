# `etcd` Bundled in a Crate

This crate provides access to an `etcd` binary for the purpose of unit testing.
The core of this library is the `etcd_bin_path` function, which gets a path to an `etcd` binary which is compatible with
this operating system and architecture.

```rust
etcd_bin_vendored::etcd_bin_path().unwrap();
```

## Platform Specific

To minimize library size, the actual binaries are packaged inside of platform-specific libraries.
For example, the [`etcd-bin-vendored-linux-amd64`][etcd-bin-vendored-linux-amd64] crate contains the binary for running
on Linux AMD64.
By default, the platform-specific crate is enabled for the detected target.
In other words, the [`etcd-bin-vendored-darwin-arm64`][etcd-bin-vendored-darwin-arm64] package is a required dependency
when you are running on MacOS with ARM64 silicon.

You can also select supported platforms manually.
For example, if you know your system is only ever ARM64 Linux, you can use the platform-specific `etcd_bin_path`
directly.
Take care with this, as the binary is always available, but it might not be runnable on your architecture.

| `target_os` | `target_arch` | `feature`       | Crate                                                                |
|-------------|---------------|-----------------|:---------------------------------------------------------------------|
| `linux`     | `x86_64`      | `linux-amd64`   | [`etcd-bin-vendored-linux-amd64`][etcd-bin-vendored-linux-amd64]     |
| `linux`     | `aarch64`     | `linux-arm64`   | [`etcd-bin-vendored-linux-arm64`][etcd-bin-vendored-linux-arm64]     |
| `linux`     | `powerpc64`   | `linux-ppc64le` | [`etcd-bin-vendored-linux-ppc64le`][etcd-bin-vendored-linux-ppc64le] |
| `linux`     | `s390x`       | `linux-s390x`   | [`etcd-bin-vendored-linux-s390x`][etcd-bin-vendored-linux-s390x]     |
| `macos`     | `x86_64`      | `macos-amd64`   | [`etcd-bin-vendored-darwin-amd64`][etcd-bin-vendored-darwin-amd64]   |
| `macos`     | `aarch64`     | `macos-arm64`   | [`etcd-bin-vendored-darwin-arm64`][etcd-bin-vendored-darwin-arm64]   |
| `windows`   | `x86_64`      | `windows-amd64` | [`etcd-bin-vendored-windows-amd64`][etcd-bin-vendored-windows-amd64] |

## Versioning

The versions of this library match the `etcd` version.
So, pulling this library at version `3.5.23` means you get the binaries for etcd release `3.5.23`.

## Caveats

### Not Suitable for Distribution

This crate is not useful for _distributing_ `etcd` to your application.
It merely pulls the `etcd` binary from its own Cargo manifest.
If you try to use this function outside of a Cargo test on the system it was built on, it will not work.

The advantage of this is that it makes it impossible to use this in production.
That is an advantage because you should not be running a critical service like `etcd` from a binary you pulled from some
random crate.
This is for unit/integration testing only.

[etcd-bin-vendored-linux-amd64]:   https://docs.rs/etcd-bin-vendored-linux-amd64/latest/etcd_bin_vendored_linux_amd64/
[etcd-bin-vendored-linux-arm64]:   https://docs.rs/etcd-bin-vendored-linux-arm64/latest/etcd_bin_vendored_linux_arm64/
[etcd-bin-vendored-linux-ppc64le]: https://docs.rs/etcd-bin-vendored-linux-ppc64le/latest/etcd_bin_vendored_linux_ppc64le/
[etcd-bin-vendored-linux-s390x]:   https://docs.rs/etcd-bin-vendored-linux-s390x/latest/etcd_bin_vendored_linux_s390x/
[etcd-bin-vendored-darwin-amd64]:  https://docs.rs/etcd-bin-vendored-darwin-amd64/latest/etcd_bin_vendored_darwin_amd64/
[etcd-bin-vendored-darwin-arm64]:  https://docs.rs/etcd-bin-vendored-darwin-arm64/latest/etcd_bin_vendored_darwin_arm64/
[etcd-bin-vendored-windows-amd64]: https://docs.rs/etcd-bin-vendored-windows-amd64/latest/etcd_bin_vendored_windows_amd64/
