//! Why use Rand crate, when you can create your on random number generator >:)
//! This is Linear congruential random number generator

/// Represents Seed, and last generated number.
/// isn't public so can't be accessed by an ignorant programmer (like me)
static mut LAST_RAND: usize = 0;

/// Used by BSD
const MULTIPLIER: usize = 214013;
/// Used by BSD
const INCREMENT: usize = 2531011;

/// also 'flushs' old random data
pub fn set_seed(seed: usize) {
    unsafe { LAST_RAND = seed }
}

pub fn generator(x_n: usize, multiplier: usize, increment: usize) -> usize {
    ((multiplier * x_n) + increment) % 2_usize.pow(31)
}

pub fn get_random() -> usize {
    let x_n = unsafe { LAST_RAND };
    let x_np1 = generator(x_n, MULTIPLIER, INCREMENT);
    unsafe { LAST_RAND = x_np1 };
    x_np1
}

pub fn get_random_max(max: usize) -> usize {
    get_random() % max
}
