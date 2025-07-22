/// The enum variants are required to be the same as the macro struct names. For example all Luffy
/// cards will be named `MonkeyDLuffy`, this is so we can deal with `{Monkey D Luffy}` syntax from
/// card text.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Character {
    Ain,
    CharlotteKatakuri,
    Jinbe,
    Koby,
    Nami,
    MonkeyDLuffy,
    Usopp,
    Shirahoshi,
}
