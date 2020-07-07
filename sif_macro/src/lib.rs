#[macro_use]
extern crate syn;
extern crate proc_macro;

use crate::case_parser::CasesFn;
use quote::quote;
use syn::{Attribute, Block, Expr, Ident, Type, Visibility};

mod case_parser;

#[proc_macro_attribute]
pub fn parameterized(
    _args: proc_macro::TokenStream,
    fn_body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let inputs = parse_macro_input!(fn_body as CasesFn);

    let visibility = inputs.fn_visibility();
    let mod_ident = inputs.fn_ident();
    let other_attributes = inputs.regular_attrs();
    let test_parameters = inputs.fn_parameters();
    let arguments = inputs.case_attrs();
    let body = inputs.fn_body();
    let test_cases = arguments
        .iter()
        .map(|exprs| exprs.values.iter().collect::<Vec<&Expr>>())
        .enumerate()
        .map(|(nth, exprs)| {
            create_test_case(
                Ident::new(&format!("case_{}", nth), inputs.fn_span()),
                &test_parameters,
                &exprs,
                body,
                visibility,
                &other_attributes,
            )
        });

    (quote! {
        #[cfg(test)]
        #visibility mod #mod_ident {
            use super::*;

            #(#test_cases)*
        }

    })
    .into()
}

fn create_test_case(
    ident: Ident,
    params: &[(&Ident, &Type)],
    exprs: &[&Expr],
    body: &Block,
    vis: &Visibility,
    attributes: &[&Attribute],
) -> proc_macro2::TokenStream {
    assert_eq!(
        params.len(),
        exprs.len(),
        "[sif_macro] A case has an insufficient amount of arguments ({} parameter(s) registered, but {} argument(s) were supplied)",
        params.len(),
        exprs.len()
    );

    let bindings = (0..params.len()).map(|i| create_binding(params[i], exprs[i]));

    quote! {
        #[test]
        #(#attributes)*
        #vis fn #ident() {
            #(#bindings)*
            #body
        }
    }
}

fn create_binding(param: (&Ident, &Type), expr: &Expr) -> proc_macro2::TokenStream {
    let (ident, typ) = param;

    quote! {
        let #ident: #typ = #expr;
    }
}
