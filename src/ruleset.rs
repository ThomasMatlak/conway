pub enum Ruleset {
    Life,
    Replicator,
    Seeds,
    LifeWithoutDeath,
    ThreeFourLife,
    Diamoeba,
    TwoByTwo,
    HighLife,
    NightAndDay,
    Morley,
    Anneal,
    DryLife,
}

#[macro_export]
macro_rules! life {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (true, 2) => true,
            (_, 3) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! replicator {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (_, 1 | 3 | 5 | 7) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! seeds {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (false, 2) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! life_without_death {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (true, _) => true,
            (false, 3) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! three_four_life {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (_, 3 | 4) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! diamoeba {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (true, 3) => true,
            (_, 5 | 6 | 7 | 8) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! two_by_two {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (false, 3 | 6) => true,
            (true, 1 | 2 | 5) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! high_life {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (false, 3 | 6) => true,
            (true, 2 | 3) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! night_and_day {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (_, 3 | 6 | 7 | 8) => true,
            (true, 4) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! morley {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (false, 3 | 6 | 8) => true,
            (true, 2 | 4 | 5) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! anneal {
    ($current_state:expr , $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (false, 4) => true,
            (true, 3 | 5) => true,
            (_, 6 | 7 | 8) => true,
            _ => false
        }
    }
}

#[macro_export]
macro_rules! dry_life {
    ($current_state:expr, $num_living_neighbors:expr) => {
        match ($current_state, $num_living_neighbors) {
            (true, 2) => true,
            (_, 3) => true,
            (false, 7) => true,
            _ => false
        }
    };
}
