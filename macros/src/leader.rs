use syn::{
    Ident, LitInt, Path, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Comma},
};

pub struct LeaderInput {
    pub name: Ident,
    pub power: LitInt,
    pub attribute: Path,
    pub colors: Vec<Path>,
    pub life: LitInt,
    pub groups: Vec<Path>,
    pub set: Path,
    pub set_card_number: LitInt,
    pub block_symbol: LitInt,
}

impl Parse for LeaderInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _ = input.parse::<Comma>()?;

        let power = input.parse()?;
        let _ = input.parse::<Comma>()?;

        let attribute = input.parse()?;
        let _ = input.parse::<Comma>()?;

        let color = if input.peek(Bracket) {
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
        let _ = input.parse::<Comma>()?;

        let life = input.parse()?;
        let _ = input.parse::<Comma>()?;

        let groups = if input.peek(Bracket) {
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
        let _ = input.parse::<Comma>()?;

        let set = input.parse()?;
        let content;
        parenthesized!(content in input);
        let set_card_number: LitInt = content.parse()?;
        let _ = input.parse::<Comma>()?;

        let block_symbol = input.parse()?;
        let _: Option<Comma> = input.parse()?; // Optional trailing comma.

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
