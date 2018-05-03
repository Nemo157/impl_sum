#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;

use std::iter::FromIterator;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Item, ExprReturn, visit::Visit, visit_mut::VisitMut, Expr, ExprCall, Path, punctuated::Punctuated, ExprPath, token::Paren, PathSegment, Ident, TraitItem, FnArg, ArgSelfRef, ArgCaptured, Pat};
use quote::ToTokens;
use quote::{quote_spanned, quote_each_token, pounded_var_names, multi_zip_expr, nested_tuples_pat};
use quote::quote;

struct Count<'a> {
    count: &'a mut usize,
}

struct InsertVariants {
    unused: Vec<Path>,
}

macro_rules! stringified_array {
    ($($i:ident)+) => ([$(stringify!($i)),+]);
}

const VARIANTS: &[&str] = &stringified_array! {
    A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
};

#[proc_macro_attribute]
pub fn impl_sum(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = match syn::parse(function).expect("failed to parse tokens as a function") {
        Item::Fn(item) => item,
        _ => panic!("#[impl_sum] can only be applied to functions"),
    };

    let mut count = 0;
    Count { count: &mut count }.visit_item_fn(&function);

    let ty = VARIANTS[count];

    let unused = VARIANTS.iter().take(count).map(|&variant| {
        Path {
            leading_colon: Some(Default::default()),
            segments: Punctuated::from_iter(vec![
                PathSegment::from("impl_sum"),
                PathSegment::from(ty),
                PathSegment::from(variant),
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

#[proc_macro_attribute]
pub fn impl_sum_impl(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item = match syn::parse(item).expect("failed to parse tokens as a trait") {
        Item::Trait(item) => item,
        _ => panic!("#[impl_sum_impl] can only be applied to traits"),
    };

    let path: Path = syn::parse(attribute).expect("failed to parse argument as a path");

    let variants = VARIANTS.iter().map(|&variant| Ident::from(variant));

    let impls = variants.clone().enumerate().skip(1).map(|(i, ty)| {
        let mut generic_parameters: Vec<_> = variants.clone().take(i).collect();
        generic_parameters.reverse();
        let generic_parameters2 = generic_parameters.iter();
        let generic_parameters3 = generic_parameters.iter();
        let first_parameter = generic_parameters.iter().next().unwrap();

        let types: Vec<_> = item.items.iter().filter_map(|item| {
            if let TraitItem::Type(ty) = item {
                Some(ty.ident)
            } else {
                None
            }
        }).collect();

        let generic_bounds = generic_parameters.iter().skip(1).map(|variant| {
            if types.is_empty() {
                quote! { #variant: #path }
            } else {
                let type_constraints = types.iter().map(|ty| quote!(#ty = #first_parameter::#ty));
                quote! { #variant: #path<#(#type_constraints),*> }
            }
        });

        let items = item.items.iter().map(|item| {
            match item {
                TraitItem::Const(_)
                | TraitItem::Macro(_)
                | TraitItem::Verbatim(_)
                => unimplemented!(),

                TraitItem::Type(ty) => {
                    let ident = ty.ident;
                    quote! { type #ident = #first_parameter::#ident; }
                }

                TraitItem::Method(method) => {
                    let ident = method.sig.ident;
                    let (self_value, self_match) = match method.sig.decl.inputs.iter().next() {
                        Some(FnArg::SelfRef(ArgSelfRef { mutability: Some(_), .. })) => {
                            (quote!(*self), quote!(ref mut e))
                        }
                        Some(FnArg::SelfRef(ArgSelfRef { mutability: None, .. })) => {
                            (quote!(*self), quote!(ref e))
                        }
                        Some(FnArg::SelfValue(_)) => {
                            (quote!(self), quote!(e))
                        }
                        _ => {
                            panic!("Non-object safe traits are not supported");
                        }
                    };
                    let sig = &method.sig;
                    let other_args: Vec<_> = method.sig.decl.inputs.iter().skip(1).map(|arg| {
                        match arg {
                            FnArg::Captured(ArgCaptured { pat: Pat::Ident(name), .. }) => {
                                name
                            }
                            _ => {
                                panic!("Only simple bindings are supported");
                            }
                        }
                    }).collect();
                    let matchers = generic_parameters.iter().map(|variant| {
                        let other_args = other_args.iter();
                        quote! {
                            ::#ty::#variant(#self_match) => #variant::#ident(e #(, #other_args)*)
                        }
                    });
                    quote! {
                        #sig {
                            match #self_value {
                                #(#matchers),*
                            }
                        }
                    }
                }
            }
        });
        quote! {
            impl<#(#generic_parameters2),*> #path
                for ::#ty<#(#generic_parameters3),*>
                where #first_parameter: #path,
                      #(#generic_bounds),*
            {
                #(#items)*
            }
        }
    });

    quote!(#(#impls)*).into()
}
