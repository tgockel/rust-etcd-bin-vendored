# `etcd` Bundled in a Crate

This crate provides access to an `etcd` binary for the purpose of unit testing.
The core of this library is the `etcd_bin_path` function, which gets a path to an `etcd` binary which is compatible with
this operating system and architecture.

```rust
# let _ =
etcd_bin_vendored::etcd_bin_path().unwrap()
# ;
```

## Caveats

### Not Suitable for Distribution

This crate is not useful for _distributing_ `etcd` to your application.
It merely pulls the `etcd` binary from its own Cargo manifest.
If you try to use this function outside of a Cargo test on the system it was built on, it will not work.

The advantage of this is that it makes it impossible to use this in production.
That is an advantage because you should not be running a critical service like `etcd` from a binary you pulled from some
random crate.
This is for unit testing only.

### Transitive Dependency Bloat

At the time of writing, the `etcd` binary is around 20 MB.
This is not huge by itself, but the base `etcd-bin-vendored` crate pulls in a prebuilt binary for every supported
architecture via its transitive dependencies (`etcd-bin-vendored-linux-x86_64`, `etcd-bin-vendored-win32`, etc.).
Even though only one of these will work for your platform, since Cargo downloads transitive dependencies before
deciding if they are enabled, these unused binaries will be downloaded.

To prevent this, you can select only supported platforms manually.
For example, if you know your system is only ever used on 64-bit Linux, you can use the platform-specific crate
`etcd-bin-vendored-linux-x86_64` and call the platform-specific `etcd_bin_path` directly.
The platform-specific functions will always return `Ok`, so it is your responsibility to make sure you use the proper
crate.

Another thing to consider doing is isolating tests which actually use `etcd` to their own crate and only pulling in the
`etcd-bin-vendored` crate for these integration tests.

## Versioning

The versions of this library match the `etcd` version.
Currently, there is a leading `0` and, due to Cargo only accepting `{MAJOR}.{MINOR}.{PATCH}`, the version of the `MINOR`
and `PATCH` are crammed into the `PATCH` field.
So, `etcd` version 3.4.34 is 0.3.434 of this library.
