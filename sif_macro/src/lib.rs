#[macro_use]
extern crate syn;
extern crate proc_macro;

use crate::case_parser::{AttributeId, CasesFn, RenameNextValue};
use quote::quote;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use syn::export::Span;
use syn::{Attribute, Block, Expr, Ident, Type, Visibility};

mod case_parser;

#[proc_macro_attribute]
pub fn parameterized(
    _args: proc_macro::TokenStream,
    fn_body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let inputs = parse_macro_input!(fn_body as CasesFn);

    let rename_table = inputs.queryable_rename_attrs();
    let rename_db = Rc::new(RefCell::new(rename_table));

    let visibility = inputs.fn_visibility();
    let mod_ident = inputs.fn_ident();
    let other_attributes = inputs.regular_attrs();
    let test_parameters = inputs.fn_parameters();
    let arguments = inputs.test_case_attrs();
    let body = inputs.fn_body();
    let test_cases = arguments
        .iter()
        .enumerate()
        .map(|(nth, exprs)| {
            let case_exprs = exprs.inner.values.iter().collect::<Vec<&Expr>>();
            let case_name =
                create_case_ident(rename_db.borrow_mut(), exprs.index, nth, inputs.fn_span());

            create_test_case(
                case_name,
                &test_parameters,
                &case_exprs,
                body,
                visibility,
                &other_attributes,
            )
        })
        .collect::<Vec<_>>();

    if rename_db.borrow().len() > 0 {
        let hint = rename_db
            .borrow()
            .iter()
            .map(|(index, item)| {
                format!(
                    "\tAttribute '{}' [stack index: {}] used incorrectly",
                    &item.ident, index
                )
            })
            .collect::<String>();

        panic!("[sif_macro] Found an unused 'rename' attribute for test '{}'; verify that you correctly put 'rename' attributes \
          on top of 'case' attributes. On top, 'rename' attributes do not stack. Hints:\nAt test fn '{}':\n{}\n\n(note: \
          stack index starts at 0, after #[parameterized] attribute)", mod_ident, mod_ident, &hint);
    }

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

type RenameDB<'map> = RefMut<'map, std::collections::HashMap<AttributeId, &'map RenameNextValue>>;

fn create_case_ident(
    mut rename_db: RenameDB, // this allows us to query 'rename' attributes on attribute index
    attribute_index: AttributeId, // defines the nth attribute, where the 0th attribute is the first attribute following #[parameterized]
    case_attribute_index: usize, // defines the nth 'case' attribute, where the 0th attribute is the first 'case' attribute following #[parameterized], and all attributes are a member of the 'case' attribute
    default_span: Span,          // default span is used when we do not rename a test case function
) -> Ident {
    // we can't have an index of 0, since a rename comes always before a case attribute
    // so if
    if attribute_index == 0 {
        create_numbered_case_ident(case_attribute_index, default_span)
    } else {
        rename_db
            .remove(&(attribute_index - 1))
            .map(|v| v.ident.clone())
            .unwrap_or_else(|| create_numbered_case_ident(case_attribute_index, default_span))
    }
}

fn create_numbered_case_ident(nr: usize, span: Span) -> Ident {
    Ident::new(&format!("case_{}", nr), span)
}
