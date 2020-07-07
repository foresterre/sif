use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Block, FnArg, Ident, ItemFn, Pat, PatIdent, PatType, Type, Visibility};

#[derive(Clone, Debug)]
pub(crate) struct CasesFn {
    attrs: Vec<SifAttribute>,
    item_fn: ItemFn,
}

impl CasesFn {
    pub(crate) fn fn_visibility(&self) -> &Visibility {
        &self.item_fn.vis
    }

    pub(crate) fn fn_ident(&self) -> &Ident {
        &self.item_fn.sig.ident
    }

    pub(crate) fn fn_parameters(&self) -> Vec<(&Ident, &Type)> {
        self.item_fn
            .sig
            .inputs
            .iter()
            .filter_map(|item| {
                if let FnArg::Typed(PatType { pat, ty, .. }) = item {
                    if let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() {
                        Some((ident, ty.as_ref()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn fn_span(&self) -> Span {
        self.item_fn.span()
    }

    pub(crate) fn fn_body(&self) -> &Block {
        self.item_fn.block.as_ref()
    }

    pub(crate) fn regular_attrs(&self) -> Vec<&Attribute> {
        self.attrs
            .iter()
            .filter_map(|item| item.attribute_as_ref())
            .collect()
    }

    pub(crate) fn case_attrs(&self) -> Vec<&CaseValues> {
        self.attrs
            .iter()
            .filter_map(|item| item.case_values_as_ref())
            .collect()
    }
}

impl Parse for CasesFn {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input
                .call(Attribute::parse_outer)?
                .into_iter()
                .map(|attr| {
                    if attr.path.is_ident("case") {
                        attr.parse_args::<CaseValues>().map(SifAttribute::Case)
                    } else {
                        Ok(SifAttribute::Regular(attr))
                    }
                })
                .collect::<Result<Vec<SifAttribute>>>()?,
            item_fn: input.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) enum SifAttribute {
    Case(CaseValues),
    Regular(Attribute),
}

impl SifAttribute {
    fn attribute_as_ref(&self) -> Option<&Attribute> {
        if let Self::Regular(attr) = self {
            Some(attr)
        } else {
            None
        }
    }

    fn case_values_as_ref(&self) -> Option<&CaseValues> {
        if let Self::Case(values) = self {
            Some(values)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CaseValues {
    pub(crate) values: Punctuated<syn::Expr, syn::token::Comma>,
}

impl Parse for CaseValues {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            values: Punctuated::parse_terminated(input)?,
        })
    }
}
