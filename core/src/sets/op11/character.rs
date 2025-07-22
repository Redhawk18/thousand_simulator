use thousand_macros::character;

use crate::{attribute::Attribute, card::Character, color::Color, group::Group, set::Set};

character!(
    Ain,
    3,
    4000,
    Attribute::Wisdom,
    1000,
    Color::Red,
    [Group::Film, Group::NeoNavy],
    Set::OP11(2),
    3
);

character!(
    Usopp,
    5,
    6000,
    Attribute::Ranged,
    2000,
    Color::Red,
    Group::StrawHatCrew,
    Set::OP11(3),
    3
);

character!(
    Kujyaku,
    1,
    0,
    Attribute::Special,
    1000,
    Color::Red,
    [Group::Navy, Group::Sword],
    Set::OP11(4),
    3
);

character!(
    Smoker,
    4,
    5000,
    Attribute::Special,
    1000,
    Color::Red,
    Group::Navy,
    Set::OP11(5),
    3
);

character!(
    Zephyr,
    5,
    6000,
    Attribute::Strike,
    1000,
    Color::Red,
    [Group::Film, Group::Navy],
    Set::OP11(6),
    3
);

character!(
    Tashigi,
    1,
    1000,
    Attribute::Slash,
    1000,
    Color::Red,
    Group::Navy,
    Set::OP11(7),
    3
);

character!(
    Doll,
    4,
    1000,
    Attribute::Strike,
    1000,
    Color::Red,
    Group::Navy,
    Set::OP11(8),
    3
);

character!(
    NicoRobin,
    3,
    4000,
    Attribute::Strike,
    1000,
    Color::Red,
    Group::StrawHatCrew,
    Set::OP11(9),
    3
);

character!(
    Hibari,
    5,
    6000,
    Attribute::Ranged,
    0,
    Color::Red,
    [Group::Navy, Group::Sword],
    Set::OP11(10),
    3
);
