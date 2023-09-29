# librrd-sys

A [bindgen](https://rust-lang.github.io/rust-bindgen/) generated wrappers for [librrd](https://oss.oetiker.ch/rrdtool/doc/librrd.en.html), provides a low-level interface to [RRDtool](https://oss.oetiker.ch/rrdtool/index.en.html).

## Tested Setup

- macOS 13.4.1
- Rust 1.72.0
- other dependencies to build librrd would be required. See
  [rrdtool-1.x/doc/rrdbuild.pod](https://github.com/oetiker/rrdtool-1.x/blob/master/doc/rrdbuild.pod) if you have any trouble.

## Build

```console
$ cargo build
```

If you would like to skip building librrd, set `LIBRRD_SYS_SKIP_BUILD` environment variable.

```console
$ LIBRRD_SYS_SKIP_BUILD=1 cargo build
```

## License

GPLv2, as same as RRDtool. See [LICENSE](LICENSE) and also [RRDtool - License](https://oss.oetiker.ch/rrdtool/license.en.html).