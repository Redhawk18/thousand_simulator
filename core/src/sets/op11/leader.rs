use thousand_macros::leader;

use crate::{Attribute, Color, Group, Set, card::Leader};

leader!(
    Koby,
    5000,
    Attribute::Strike,
    [Color::Red, Color::Black],
    4,
    [Group::Navy, Group::Sword],
    Set::OP11(1),
    3,
);

leader!(
    Jinbe,
    5000,
    Attribute::Strike,
    Color::Green,
    5,
    [Group::FishMan, Group::StrawHatCrew],
    Set::OP11(21),
    3
);

leader!(
    Shirahoshi,
    5000,
    Attribute::Wisdom,
    [Color::Green, Color::Yellow],
    5,
    [Group::Merfolk, Group::FishmanIsland],
    Set::OP11(22),
    3
);

impl Leader for Shirahoshi {
    fn can_attack() -> bool {
        false
    }
}

leader!(
    MonkeyDLuffy,
    6000,
    Attribute::Strike,
    [Color::Blue, Color::Purple],
    3,
    Group::StrawHatCrew,
    Set::OP11(40),
    3
);

leader!(
    Nami,
    5000,
    Attribute::Special,
    [Color::Blue, Color::Yellow],
    4,
    Group::StrawHatCrew,
    Set::OP11(41),
    3
);

leader!(
    CharlotteKatakuri,
    5000,
    Attribute::Strike,
    Color::Purple,
    5,
    Group::BigMomPirates,
    Set::OP11(62),
    3
);
