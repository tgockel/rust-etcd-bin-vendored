# `etcd` Bundled in a Crate

This crate provides access to an `etcd` binary for the purpose of unit testing.
The core of this library is the `etcd_bin_path` function, which gets a path to an `etcd` binary which is compatible with
this operating system and architecture.

```rust
etcd_bin_vendored::etcd_bin_path().unwrap();
```

This is meant to be used as the program for an [`std::process::Command`].
To quickly spin up an `etcd` server for testing purposes, do something like this:

```rust
let etcd_path = etcd_bin_vendored::etcd_bin_path()
    .expect("etcd-bin-vendored should provide a valid path");
let client_port = 2379u16; // <- randomize it
let mut command = std::process::Command::new(etcd_path);
command
    .arg("--listen-client-urls")
    .arg(format!("http://127.0.0.1:{client_port}"))
    .arg("--advertise-client-urls")
    .arg(format!("http://127.0.0.1:{client_port}"));
let mut process = command.spawn().expect("failed to launch etcd");

// Do things with a client library
println!("etcd running -- listening at 'http://127.0.0.1:{client_port}'");

// Error handling which makes sense to you, probably in a `Drop` impl
process.kill().expect("failed to kill etcd");
process.wait().expect("waiting for etcd to stop was not successful");
```

Note that the code above will cause `etcd` to use the current working directory as the data directory, so you will see a
directory named `default.etcd` appear wherever you ran this from.
To prevent this, you can use [`tempdir`][tempdir] and the `--data-dir` argument so you don't end up testing with stale
data.
It is also good practice to pick a random unused `client_port` so you can run tests in parallel.
There is a good argument for moving these to a generic helper library.

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

| `target_os` | `target_arch` | `feature`         | Crate                                                                |
|-------------|---------------|-------------------|:---------------------------------------------------------------------|
| `linux`     | `x86_64`      | `linux-x86_64`    | [`etcd-bin-vendored-linux-amd64`][etcd-bin-vendored-linux-amd64]     |
| `linux`     | `aarch64`     | `linux-aarch64`   | [`etcd-bin-vendored-linux-arm64`][etcd-bin-vendored-linux-arm64]     |
| `linux`     | `powerpc64`   | `linux-powerpc64` | [`etcd-bin-vendored-linux-ppc64le`][etcd-bin-vendored-linux-ppc64le] |
| `macos`     | `x86_64`      | `macos-x86_64`    | [`etcd-bin-vendored-darwin-amd64`][etcd-bin-vendored-darwin-amd64]   |
| `windows`   | `x86_64`      | `windows-x86_64`  | [`etcd-bin-vendored-windows-amd64`][etcd-bin-vendored-windows-amd64] |

> **Note: Naming**
>
> There is a bit of incongruity between the platform-specific feature names and their respective crate names; e.g.: the
> `macos-aarch64` feature maps to the `etcd-bin-vendored-darwin-arm64` crate.
> The reason for this is the features map to the Rust conventions for
> [`target_os`](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os) and
> [`target_arch`](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch), while the crates map to
> [`GOOS` and `GOARCH` platform conventions](https://pkg.go.dev/internal/platform), which `etcd` uses directly.
> My thinking is that someone using the meta-crate will be more familiar with Rust conventions, while someone looking to
> pull a specific binary will be more familiar with Go conventions.

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

### Security

This crate gives you a binary blob ultimately downloaded from [`etcd` releases][etcd-releases] on GitHub.
Before running anything changing library versions, you should verify that the binary downloaded by this library matches
what you find on the release.
Note that this library gives the executable inside the archive (`etcd`/`etcd.exe`), not the whole archive, so the SHA
sums listed on GitHub will not match; you have to download the archive, unpack it, and do this yourself.

If an `etcd` release gets yanked for security reasons, I will also yank it from [Crates][crates].
If I have not done this, [open an issue][new-issue] with a "security" tag.

[crates]:                          https://crates.io/
[etcd-bin-vendored-linux-amd64]:   https://docs.rs/etcd-bin-vendored-linux-amd64/latest/etcd_bin_vendored_linux_amd64/
[etcd-bin-vendored-linux-arm64]:   https://docs.rs/etcd-bin-vendored-linux-arm64/latest/etcd_bin_vendored_linux_arm64/
[etcd-bin-vendored-linux-ppc64le]: https://docs.rs/etcd-bin-vendored-linux-ppc64le/latest/etcd_bin_vendored_linux_ppc64le/
[etcd-bin-vendored-linux-s390x]:   https://docs.rs/etcd-bin-vendored-linux-s390x/latest/etcd_bin_vendored_linux_s390x/
[etcd-bin-vendored-darwin-amd64]:  https://docs.rs/etcd-bin-vendored-darwin-amd64/latest/etcd_bin_vendored_darwin_amd64/
[etcd-bin-vendored-darwin-arm64]:  https://docs.rs/etcd-bin-vendored-darwin-arm64/latest/etcd_bin_vendored_darwin_arm64/
[etcd-bin-vendored-windows-amd64]: https://docs.rs/etcd-bin-vendored-windows-amd64/latest/etcd_bin_vendored_windows_amd64/
[etcd-releases]:                   https://github.com/etcd-io/etcd/releases
[new-issue]:                       https://github.com/tgockel/rust-etcd-bin-vendored/issues/new
[tempdir]:                         https://docs.rs/tempdir/latest/tempdir/
