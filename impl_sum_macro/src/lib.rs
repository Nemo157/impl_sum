#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;

use std::iter::FromIterator;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Item, Ident, ExprReturn, visit::Visit, visit_mut::VisitMut, Expr, ExprCall, Path, punctuated::Punctuated, ExprPath, token::Paren, PathSegment};
use quote::ToTokens;

struct Count<'a> {
    count: &'a mut usize,
}

struct InsertVariants {
    unused: Vec<Path>,
}

#[proc_macro_attribute]
pub fn impl_sum(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = match syn::parse(function).expect("failed to parse tokens as a function") {
        Item::Fn(item) => item,
        _ => panic!("#[impl_sum] can only be applied to functions"),
    };

    let mut count = 0;
    Count { count: &mut count }.visit_item_fn(&function);

    let ty = Ident::from(format!("Either{}", count));

    const VARIANTS: &[&str] = &["A", "B"];

    let unused = VARIANTS.iter().take(count).map(|&variant| {
        Path {
            leading_colon: Some(Default::default()),
            segments: Punctuated::from_iter(vec![
                PathSegment::from(Ident::from("impl_sum")),
                PathSegment::from(ty),
                PathSegment::from(Ident::from(variant)),
            ]),
        }
    }).collect();

    InsertVariants { unused }.visit_item_fn_mut(&mut function);

    function.into_tokens().into()
}

impl<'a, 'ast> Visit<'ast> for Count<'a> {
    fn visit_expr_return(&mut self, _expr_return: &'ast ExprReturn) {
        *self.count += 1;
    }

    fn visit_item(&mut self, _item: &'ast Item) {
    }
}

impl VisitMut for InsertVariants {
    fn visit_expr_return_mut(&mut self, expr_return: &mut ExprReturn) {
        let variant = self.unused.pop().expect("exceeded max number of variants");

        expr_return.expr = expr_return.expr.take().map(|expr| {
            Box::new(Expr::Call(ExprCall {
                attrs: vec![],
                func: Box::new(Expr::Path(ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: variant,
                })),
                paren_token: Paren(Span::call_site()),
                args: Punctuated::from_iter(Some(*expr)),
            }))
        });
    }

    fn visit_item_mut(&mut self, _item: &mut Item) {
    }
}
