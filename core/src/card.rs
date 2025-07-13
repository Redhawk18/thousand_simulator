pub trait Character {}

pub trait Event {}

pub trait Leader {
    fn can_attack() -> bool {
        true
    }

    // fn your_turn_once_per_turn() {}
}

pub trait Stage {}
