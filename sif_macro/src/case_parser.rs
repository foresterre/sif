use proc_macro2::Span;
use std::collections::HashMap;
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

    pub(crate) fn test_case_attrs(&self) -> Vec<&CaseEnfold> {
        self.attrs
            .iter()
            .filter_map(|item| item.case_values_as_ref())
            .collect()
    }

    pub(crate) fn regular_attrs(&self) -> Vec<&Attribute> {
        self.attrs
            .iter()
            .filter_map(|item| item.attribute_as_ref())
            .collect()
    }

    pub(crate) fn queryable_rename_attrs(&self) -> HashMap<AttributeId, &RenameNextValue> {
        self.attrs
            .iter()
            .filter_map(|item| item.rename_next_as_ref())
            .map(|item| (item.index, &item.inner))
            .collect()
    }
}

impl Parse for CasesFn {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input
                .call(Attribute::parse_outer)?
                .into_iter()
                .enumerate()
                .map(|(nth, attr)| {
                    if attr.path.is_ident("case") {
                        attr.parse_args::<CaseValues>()
                            .map(|val| SifAttribute::Case(val.into_enfold(nth)))
                    } else if attr.path.is_ident("rename") {
                        attr.parse_args::<RenameNextValue>()
                            .map(|val| SifAttribute::RenameNext(val.into_enfold(nth)))
                    } else {
                        Ok(SifAttribute::Regular(attr))
                    }
                })
                .collect::<Result<Vec<SifAttribute>>>()?,
            item_fn: input.parse()?,
        })
    }
}

pub(crate) type AttributeId = usize;

#[derive(Clone, Debug)]
pub(crate) enum SifAttribute {
    Case(CaseEnfold),
    Regular(Attribute),
    RenameNext(RenameNextEnfold),
}

impl SifAttribute {
    fn case_values_as_ref(&self) -> Option<&CaseEnfold> {
        if let SifAttribute::Case(values) = self {
            Some(values)
        } else {
            None
        }
    }

    fn attribute_as_ref(&self) -> Option<&Attribute> {
        if let SifAttribute::Regular(attr) = self {
            Some(attr)
        } else {
            None
        }
    }

    fn rename_next_as_ref(&self) -> Option<&RenameNextEnfold> {
        if let SifAttribute::RenameNext(val) = self {
            Some(val)
        } else {
            None
        }
    }
}

pub(crate) trait IntoEnfold<T>: Sized {
    /// Transforms the value into an annotated (with an id) variant of itself
    fn into_enfold(self, index: AttributeId) -> T;
}

macro_rules! enfold {
    ($name: ident, $matter_ty: ty) => {
        /// A wrapped value struct to have a single (container) type as enum value for `SifAttribute`
        /// types which have to be annotated with attribute ids
        #[derive(Clone, Debug)]
        pub(crate) struct $name {
            pub(crate) inner: $matter_ty,

            /// The nth attribute, use for renames, to decide which case attribute should be renamed
            /// (the `nth+1` if it exists)
            pub(crate) index: AttributeId,
        }

        impl IntoEnfold<$name> for $matter_ty {
            fn into_enfold(self, index: usize) -> $name {
                $name { inner: self, index }
            }
        }
    };
}

enfold!(CaseEnfold, CaseValues);
enfold!(RenameNextEnfold, RenameNextValue);

#[derive(Clone, Debug)]
pub(crate) struct CaseValues {
    pub(crate) span: Span,
    pub(crate) values: Punctuated<syn::Expr, syn::token::Comma>,
}

impl Parse for CaseValues {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            span: input.span(),
            values: Punctuated::parse_terminated(input)?,
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RenameNextValue {
    pub(crate) span: Span,
    pub(crate) ident: syn::Ident,
}

impl Parse for RenameNextValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            span: input.span(),
            ident: input.parse()?,
        })
    }
}
