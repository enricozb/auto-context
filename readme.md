## auto-context

Auto-add context to anyhow errors (without nightly).

## Example

```rust
use anyhow::{Context, Result};

struct Test {}

#[auto_context::auto_context]
impl Test {
  fn some_method(self) -> Result<()> {
    anyhow::bail!("some error")
  }

  fn some_function(_: i32) -> Result<()> {
    Test {}.some_method()?;

    Ok(())
  }
}

#[auto_context::auto_context]
fn main() -> Result<()> {
  Test::some_function(123)?;

  Ok(())
}
```
fails with
```text
Error: Test::some_function(..) @ auto-context/src/lib.rs::23

Caused by:
    0: .some_method() @ auto-context/src/lib.rs::15
    1: some error
```

## Details

The [auto_context][1] proc macro can be used to annotate any item. This
includes functions, methods, and struct/trait impls.

Context is added to every [try expression] (every use of a `?`). Different
kinds of expressions result in different context formats:

- method calls: `.method_name(args)`
- function calls: `some::func(args)`
- identifiers: `xyz`
- expression calls: `(.. some expr ..)`

where `args` is `..` if arguments are present and is empty otherwise.

[1]: https://docs.rs/auto-context
