[![Build Status](https://travis-ci.org/bradleygibson/xargo-lld-wrapper.svg?branch=master)](https://travis-ci.org/bradleygibson/xargo-lld-wrapper)

## Description
A utility to enable one-command cross-compilation using xargo and steed on Mac OS X (eg. for Raspberry Pi 3: `xargo build --bin hello --target armv7-unknown-linux-steedeabihf`).

Use this application in place of lld (`ld.lld`) on your Mac OS X system.  Arguments provided by Xargo to the linker will be "fixed" to be compatible with lld, as per [this discussion](https://users.rust-lang.org/t/solved-cross-compilation-from-mac-os-x-using-steed/9709/6).  `xargo-lld-wrapper` will then invoke lld with those fixed arguments.

## Instructions
You will have to [build and install lld](https://lld.llvm.org/#build) from source.  Once `ld.lld` is on your path, xargo-lld-wrapper will be able to invoke it.

To tell Xargo to use `xargo-lld-wrapper` add the following to your ~/.cargo/config file:
```
[target.armv7-unknown-linux-steedeabihf]
linker = "xargo-lld-wrapper"
```
where `armv7-unknown-linux-steedeabihf` matches the `xargo build --target` you are are building for.  See [these instructions](https://github.com/japaric/steed) **"On Other Systems"** for fuller details.  If `xargo-lld-wrapper` is not on your $PATH, you can prefix it in `~/.cargo/config` with an absolute path (eg. `linker = "$(HOME)/.../xargo-lld-wrapper/target/release/xargo-lld-wrapper"`).

To build `xargo-lld-wrapper`:
- `git clone https://github.com/bradleygibson/xargo-lld-wrapper.git`
- `cd xargo-lld-wrapper`
- `cargo build --release`

Your new executable binary will be found at `./target/release/xargo-lld-wrapper`.

## License

Licensed under your choice of either:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

&lt;YEAR&gt; = 2017, &lt;COPYRIGHT HOLDER&gt; = Bradley Gibson.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.