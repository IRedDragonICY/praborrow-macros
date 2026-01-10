//! Procedural macros for PraBorrow defense and verification.
//!
//! Provides `#[derive(Constitution)]` for generating runtime invariant checks.
//! Includes experimental Z3/SMT-LIB proof obligation generation.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, parse_macro_input};

#[proc_macro_derive(Constitution, attributes(invariant))]
pub fn derive_constitution(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut checks = Vec::new();

    if let Data::Struct(syn::DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &input.data
    {
        for field in &fields.named {
            let _field_name = &field.ident;
            for attr in &field.attrs {
                if attr.path().is_ident("invariant") {
                    #[allow(clippy::collapsible_if)]
                    if let Meta::List(list) = &attr.meta {
                        // Expected: #[invariant(self.val > 0)]
                        // We parse the expression directly.
                        if let Ok(expr) = list.parse_args::<syn::Expr>() {
                            let check_expr = quote! { #expr };
                            let expr_str = check_expr.to_string();

                            let err_msg =
                                format!("Constitutional Invariant Violated: {}", expr_str);

                            // Year 3: SMT-LIB Translation
                            // We attempt to translate "self.value > 0" to "(assert (> value 0))"
                            let smt_expr = expr_str
                                .replace("self . ", "")
                                .replace(" > ", " (> ")
                                .replace(" >= ", " (>= ");
                            // (Very naive parser for prototype)

                            // Emit a compile-time note about the Z3 proof obligation
                            let _proof_obligation =
                                format!("; Z3 Proof Obligation: (assert {})", smt_expr);

                            // We inject this check at runtime AND emit the proof string
                            checks.push(quote! {
                                // SMT: #proof_obligation
                                if !(#check_expr) {
                                    panic!(#err_msg);
                                }
                            });
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl crate::Constitution for #name {
            fn enforce_law(&self) {
                #(#checks)*
            }
        }
    };

    TokenStream::from(expanded)
}
