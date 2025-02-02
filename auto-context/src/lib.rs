//! Auto-add context to anyhow errors (without nightly).
//!
//! # Example
//!
//! ```no_run
//! use anyhow::{Context, Result};
//!
//! struct Test {}
//!
//! #[auto_context::auto_context]
//! impl Test {
//!   fn some_method(self) -> Result<()> {
//!     anyhow::bail!("some error")
//!   }
//!
//!   fn some_function(_: i32) -> Result<()> {
//!     Test {}.some_method()?;
//!
//!     Ok(())
//!   }
//! }
//!
//! #[auto_context::auto_context]
//! fn main() -> Result<()> {
//!   Test::some_function(123)?;
//!
//!   Ok(())
//! }
//! ```
//! fails with
//! ```text
//! Error: Test::some_function(..) @ auto-context/src/lib.rs::23
//!
//! Caused by:
//!     0: .some_method() @ auto-context/src/lib.rs::15
//!     1: some error
//! ```
//!
//! # Details
//!
//! The [`macro@auto_context`] proc macro can be used to annotate any item. This
//! includes functions, methods, and struct/trait impls.
//!
//! Context is added to every [try expression] (every use of a `?`). Different
//! kinds of expressions result in different context formats:
//!
//! - method calls: `.method_name(args)`
//! - function calls: `some::func(args)`
//! - identifiers: `xyz`
//! - expression calls: `(.. some expr ..)`
//!
//! where `args` is `..` if arguments are present and is empty otherwise.
//!
//! [try expression]: syn::ExprTry

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, visit_mut::VisitMut, Expr, Item, Path};

struct AutoContext;

impl VisitMut for AutoContext {
  fn visit_expr_mut(&mut self, expr: &mut Expr) {
    if let Expr::Try(expr_try) = expr {
      let context = anyhow_context(&expr_try.expr);
      let inner = &expr_try.expr;
      let span = expr_try.question_token.spans[0];

      // thank you @t6 for thinking of `quote_spanned!`.
      *expr = syn::parse_quote_spanned! { span=>
          (#inner).context(format!("{} @ {}::{}", #context, file!(), line!()))?
      };
    }
    syn::visit_mut::visit_expr_mut(self, expr);
  }
}

// Converts a path to a string roughly representing the source it came from.
fn path_to_string(path: &Path) -> String {
  path
    .segments
    .iter()
    .map(|s| format!("{}", s.ident))
    .collect::<Vec<String>>()
    .join("::")
}

/// Returns a best-effort context for an expression.
fn anyhow_context(expr: &Expr) -> String {
  match expr {
    Expr::MethodCall(method_call) => {
      let name = method_call.method.to_string();
      let args = if method_call.args.is_empty() { "" } else { ".." };

      format!(".{name}({args})")
    }

    Expr::Call(call) => {
      if let Expr::Path(path) = &*call.func {
        let path = path_to_string(&path.path);
        let args = if call.args.is_empty() { "" } else { ".." };

        format!("{path}({args})")
      } else {
        "(.. some expression ..)".to_string()
      }
    }

    Expr::Path(path) => path_to_string(&path.path),

    _ => "(.. some expr ..)".to_string(),
  }
}

/// Auto-adds sensible context to [anyhow] errors.
///
/// Different expressions result in different context formats:
///
/// See the [module documentation] for examples.
///
/// [anyhow]: https://docs.rs/anyhow
/// [module documentation]: self
#[proc_macro_attribute]
pub fn auto_context(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut item = parse_macro_input!(item as Item);

  AutoContext.visit_item_mut(&mut item);

  TokenStream::from(quote! {
      #item
  })
}
