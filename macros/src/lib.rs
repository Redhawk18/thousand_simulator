mod character;
mod event;
mod leader;
mod stage;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use character::CharacterInput;
use event::EventInput;
use leader::LeaderInput;
use stage::StageInput;

#[proc_macro]
pub fn character(input: TokenStream) -> TokenStream {
    let character = parse_macro_input!(input as CharacterInput);

    let name = character.name;
    let cost = character.cost;
    let power = character.power;
    let attribute = character.attribute;
    let counter = character.counter;
    let color = character.color;
    let groups = character.groups;
    let set = character.set;
    let set_card_number = character.set_card_number;
    let block_symbol = character.block_symbol;

    let groups_tokens = quote! { #(#groups),* };
    let set_tokens = quote! { #set(#set_card_number) };

    let name_string = name.to_string();

    let doc = format!("{}'s character card with a blue back.", name_string);
    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name {
            cost: u8,
            power: u32,
            attribute: Attribute,
            counter: u16,
            color: Color,
            groups: Vec<Group>,
            set: Set,
            block_symbol: u8,
        }

        impl #name {
            pub fn new() -> Self {
                Self {
                    cost: #cost,
                    power: #power,
                    attribute: #attribute,
                    counter: #counter,
                    color: #color,
                    groups: vec![#groups_tokens],
                    set: #set_tokens,
                    block_symbol: #block_symbol,
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #name_string)
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn event(input: TokenStream) -> TokenStream {
    let event = parse_macro_input!(input as EventInput);

    let name = event.name;
    let cost = event.cost;
    let color = event.color;
    let groups = event.groups;
    let set = event.set;
    let set_card_number = event.set_card_number;
    let block_symbol = event.block_symbol;

    let groups_tokens = quote! { #(#groups),* };
    let set_tokens = quote! { #set(#set_card_number) };

    let name_string = name.to_string();

    let doc = format!("{}'s event card.", name_string);
    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name {
            cost: u8,
            color: Color,
            groups: Vec<Group>,
            set: Set,
            block_symbol: u8,
        }

        impl #name {
            pub fn new() -> Self {
                Self {
                    cost: #cost,
                    color: #color,
                    groups: vec![#groups_tokens],
                    set: #set_tokens,
                    block_symbol: #block_symbol,
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #name_string)
            }
        }
    }
    .into()
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

    let name_string = name.to_string();

    let doc = format!("{}'s leader card with a red back.", name_string);
    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name {
            power: u32,
            attribute: Attribute,
            colors: Vec<Color>,
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
                    colors: vec![#colors_tokens],
                    life: #life,
                    groups: vec![#groups_tokens],
                    set: #set_tokens,
                    block_symbol: #block_symbol,
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #name_string)
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn stage(input: TokenStream) -> TokenStream {
    let stage = parse_macro_input!(input as StageInput);

    let name = stage.name;
    let cost = stage.cost;
    let color = stage.color;
    let groups = stage.groups;
    let set = stage.set;
    let set_card_number = stage.set_card_number;
    let block_symbol = stage.block_symbol;

    let groups_tokens = quote! { #(#groups),* };
    let set_tokens = quote! { #set(#set_card_number) };

    let name_string = name.to_string();

    let doc = format!("{}'s event card.", name_string);
    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name {
            cost: u8,
            color: Color,
            groups: Vec<Group>,
            set: Set,
            block_symbol: u8,
        }

        impl #name {
            pub fn new() -> Self {
                Self {
                    cost: #cost,
                    color: #color,
                    groups: vec![#groups_tokens],
                    set: #set_tokens,
                    block_symbol: #block_symbol,
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #name_string)
            }
        }
    }
    .into()
}
