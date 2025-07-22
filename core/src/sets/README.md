# Sets

This is how all cards are hardcoded and scripted for the card engine to function.

## Usage

Cards can be generated like so with macros 
```rs
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
```
Any custom scripting the card need is enabled through the `card::Character` trait for character cards and vice versa.

The reason we want to avoid embedding art or text in the application is due to copyright. As my current understanding of copyright, any images, text, or artwork for the copyrighted game is not usable in the core card engine. Those questionable materials will have to be place on a web server the application can request that infomation from. Doing it this way comes with a few benifits.

1. We can support more than the offical languages the game supports, via fan translators we can post unoffical but accurate translations online for the game to use and even if the card art is in a different language the text can always be in a users supported language.
2. Avoiding a Cease and Desist, the legal ground for striking down a project that draws clear lines between copyrighted and non-copyrighted materials is hard. It can always be done, but the design of the software is also legally acceptable in court if needed.
3. Added flexablity, updates are instant if the content is sent from a server. All that is required is a hash to make sure the user's cache is upto date.
