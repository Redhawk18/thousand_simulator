use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitInt, Path, Token, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

struct LeaderInput {
    name: Ident,
    power: LitInt,
    attribute: Path,
    colors: Vec<Path>,
    life: LitInt,
    groups: Vec<Path>,
    set: Path,
    set_card_number: LitInt,
    block_symbol: LitInt,
}

impl Parse for LeaderInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _ = input.parse::<Token![,]>()?;

        let power = input.parse()?;
        let _ = input.parse::<Token![,]>()?;

        let attribute = input.parse()?;
        let _ = input.parse::<Token![,]>()?;

        let color = if input.peek(syn::token::Bracket) {
            // Case: `[Color::Red, Color::Yellow]` (bracketed list)
            let content;
            bracketed!(content in input);
            Punctuated::<Path, Comma>::parse_terminated(&content)?
                .into_iter()
                .collect()
        } else {
            // Case: Single `Color::Red` (not in brackets)
            vec![input.parse()?]
        };
        let _ = input.parse::<Token![,]>()?;

        let life = input.parse()?;
        let _ = input.parse::<Token![,]>()?;

        let groups = if input.peek(syn::token::Bracket) {
            // Case: `[Group::Fishmen, Group::StawHatCrew]` (bracketed list)
            let content;
            bracketed!(content in input);
            Punctuated::<Path, Comma>::parse_terminated(&content)?
                .into_iter()
                .collect()
        } else {
            // Case: Single `Group::StawHatCrew` (not in brackets)
            vec![input.parse()?]
        };
        let _ = input.parse::<Token![,]>()?;

        let set = input.parse()?;
        let content;
        parenthesized!(content in input);
        let set_card_number: LitInt = content.parse()?;
        let _ = input.parse::<Token![,]>()?;

        let block_symbol = input.parse()?;
        let _: Option<Token![,]> = input.parse()?; // Optional trailing comma.

        Ok(LeaderInput {
            name,
            power,
            attribute,
            colors: color,
            life,
            groups,
            set,
            set_card_number,
            block_symbol,
        })
    }
}

#[proc_macro]
pub fn leader(input: TokenStream) -> TokenStream {
    let leader = parse_macro_input!(input as LeaderInput);

    let name = leader.name;
    let power = leader.power;
    let attribute = leader.attribute;
    let colors = leader.colors;
    let life = leader.life;
    let groups = leader.groups;
    let set = leader.set;
    let set_card_number = leader.set_card_number;
    let block_symbol = leader.block_symbol;

    let colors_tokens = quote! { #(#colors),* };
    let groups_tokens = quote! { #(#groups),* };
    let set_tokens = quote! { #set(#set_card_number) };

    // let doc = format!("{}'s leader card with a red back.", name.to_string());
    quote! {
        // #[doc = #doc]
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct #name {
            power: u32,
            attribute: Attribute,
            color: Vec<Color>,
            life: u8,
            groups: Vec<Group>,
            set: Set,
            block_symbol: u8,
        }

        impl #name {
            pub fn new() -> Self {
                Self {
                    power: #power,
                    attribute: #attribute,
                    color: vec![#colors_tokens],
                    life: #life,
                    groups: vec![#groups_tokens],
                    set: #set_tokens,
                    block_symbol: #block_symbol,
                }
            }
        }
    }
    .into()
}
