#[cfg(test)]
mod tests {
  use anyhow::{Context, Result};

  struct Test {}

  #[auto_context::auto_context]
  impl Test {
    fn fail(self) -> Result<()> {
      anyhow::bail!("fail")
    }

    fn method_call(self) -> Result<()> {
      self.fail()?;

      Ok(())
    }

    fn function_call() -> Result<()> {
      Test {}.method_call()?;

      Ok(())
    }

    fn identifier() -> Result<()> {
      let res = Self::function_call();
      res?;

      Ok(())
    }
  }

  #[auto_context::auto_context]
  fn some_expression() -> Result<()> {
    let res = (Test::identifier(), ());
    res.0?;

    Ok(())
  }

  fn assert_message(res: Result<()>, msg: impl Into<String>) {
    let Err(err) = res else { panic!("expected error") };

    assert_eq!(format!("{:?}", err), msg.into())
  }

  #[test]
  fn test_all() {
    let msg = indoc::indoc! {"
      (.. some expr ..) @ tests/src/lib.rs::36

      Caused by:
          0: res @ tests/src/lib.rs::27
          1: .method_call() @ tests/src/lib.rs::20
          2: .fail() @ tests/src/lib.rs::14
          3: fail
    "}
    .trim();

    assert_message(some_expression(), msg)
  }
}
