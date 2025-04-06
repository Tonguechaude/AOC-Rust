// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    // Calculate the ratio of good deeds to total deeds
    // Any ratio greater than or equal to 0.75 is considered nice
    // e.g. 10 good deeds and 2 bad deeds =
    // (10 * 1) / ((10 * 1) + (2 * 2)) = 10 / 14 = 0.714... (not nice)
    // If both good and bad deeds are 0, the child is naughty

    if bad_deeds as f32 == 0f32 && good_deeds as f32 == 0f32 {
        return false;
    }

    let total: f32 = (good_deeds as f32 * GOOD_WEIGHT)
        / ((good_deeds as f32 * GOOD_WEIGHT) + (bad_deeds as f32 * BAD_WEIGHT));
    if total < 0.75f32 { false } else { true }
}

pub fn main() -> () {}
