use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;

#[unsafe(no_mangle)]
pub extern "C" fn violet_irand(min: i32, max: i32) -> i32 {
    if min > max {
        return min;
    }
    rand::rng().random_range(min..=max)
}

#[unsafe(no_mangle)]
pub extern "C" fn violet_urand(min: u32, max: u32) -> u32 {
    if min > max {
        return min;
    }
    rand::rng().random_range(min..=max)
}

#[unsafe(no_mangle)]
pub extern "C" fn violet_urand32() -> u32 {
    rand::rng().next_u32()
}

#[unsafe(no_mangle)]
pub extern "C" fn violet_frand(min: f32, max: f32) -> f32 {
    if min > max {
        return min;
    }
    rand::rng().random_range(min..=max)
}

// return double value as f64
#[unsafe(no_mangle)]
pub extern "C" fn violet_rand_chance() -> f64 {
    rand::rng().random_range(0.0..100.0)
}

// return double value as f64
#[unsafe(no_mangle)]
pub extern "C" fn violet_rand_norm() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn violet_urand_weighted(count: usize, chances: *const f64) -> u32 {
    let weights = unsafe {
        std::slice::from_raw_parts(chances, count)
    };
    
    if let Ok(dist) = WeightedIndex::new(weights) {
        return rand::rng().sample(dist) as u32;
    }

    0 // fallback if some error
}