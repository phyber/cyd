# `cyd`

`cyd` (Convert Your Data, why is naming things so hard) is a small Rust program
to convert between a few serialisation formats. Currently, it can convert
between:

  - JSON
  - TOML
  - YAML

It reads from `STDIN` and outputs on `STDOUT`.

`cyd` requires at least Rust 1.31.

## Example Usage

Converting our `Cargo.toml` to YAML

```shell
cyd --input toml --output yaml < Cargo.toml
```

## Caveats

  - Sometimes it may not be possible to convert from one format to another, in
    these cases, `cyd` should output an error that makes sense to someone,
    somewhere.
  - We couldn't use the `serde_transcode` crate since not all formats exported
    the `Serialize` and `Deserialize` traits.
  - We couldn't consistently use the `from_reader` and `to_writer` methods as
    not all formats supported these methods.
  - The code feels a little repetitive, this might get fixed once I level up in
    Rust some more.

## License

MIT
