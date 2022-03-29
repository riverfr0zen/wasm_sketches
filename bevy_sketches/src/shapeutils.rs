use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::thread_rng;
use rand::Rng;


/// Function to safeguard against invalid arguments when generating random range
fn gen_random_safely(start: f32, end: f32) -> f32 {
    let mut rng = thread_rng();

    // if start >= end {
    //     debug!("!!! s {}, e {}", start, end);
    //     return end;
    // }
    // return rng.gen_range(start..end);

    // Returning as i16 might somewhat improve results (not as many small floating point
    // variations that translate to screen coord variations). But dunno if it does much good.
    let start = start as i16;
    let end = end as i16;

    if start >= end {
        debug!("!!! s {}, e {}", start, end);
        // return end;
        return end as f32;
    }
    return rng.gen_range(start..end) as f32;
}


/// @TODO Good enough for now, but can be improved.
///
/// Currently the randomization reaches the end of range in a quadrant too quickly,
/// so even if you increase number of segments, this function reaches the end of the range sooner.
///
/// So it doesn't work well for a large number of segments.
///
/// One idea for improvement is to break down each quadrant to sub-ranges for gen_range().
pub fn random_shape_builder(max_x: f32, max_y: f32, max_segments: u8) -> PathBuilder {
    let mut rng = thread_rng();

    let num_segments = rng.gen_range(3..=max_segments);
    let mut path_builder = PathBuilder::new();

    let mut last_x: f32 = 0.0;
    let mut last_y: f32 = 0.0;
    let mut current_quad = 1;

    // @HINT
    // Using an underscore to discard the iterator value since it's not being used
    for _i in 1..=num_segments {
        let segment_place: f32 = _i as f32 / num_segments as f32;

        if segment_place <= 0.25 {
            if last_x == 0.0 {
                debug!("entered quad 1");
                last_x = gen_random_safely(-max_x, 0.0);
                last_y = gen_random_safely(0.0, max_y);
                path_builder.move_to(Vec2::new(last_x, last_y));
            } else {
                last_x = gen_random_safely(last_x, 0.0);
                // For quads 1 & 3 using `last_y` seems less necessary in terms of
                // edges crossing each other. Crossing does occur, but much less
                // frequently. However, using it does eliminate all crossing, seemingly
                // at the cost of more "conservative shapes" (but I have not verified it).
                last_y = gen_random_safely(last_y, max_y);
                // last_y = rng.gen_range(0.0..winsetup.max_y);
                path_builder.line_to(Vec2::new(last_x, last_y));
            }
            debug!(
                "---i: {}, segment_place: {}, x: {}, y: {}",
                _i, segment_place, last_x, last_y
            );
            continue;
        }

        if segment_place > 0.25 && segment_place <= 0.5 {
            if current_quad < 2 {
                debug!("entered quad 2");
                last_x = gen_random_safely(0.0, max_x);
                last_y = gen_random_safely(0.0, max_y);
                path_builder.line_to(Vec2::new(last_x, last_y));
            } else {
                last_x = gen_random_safely(last_x, max_x);
                // In quads 2 & 4, where the horizontal direction of the shape generation
                // changes on the next quadrant, it seems that using `last_y` in the range
                // makes better shapes (edges don't cross)
                //
                // last_y = rng.gen_range(0.0..winsetup.max_y);
                last_y = gen_random_safely(0.0, last_y);
                path_builder.line_to(Vec2::new(last_x, last_y));
            }
            debug!(
                "---i: {}, segment_place: {}, x: {}, y: {}",
                _i, segment_place, last_x, last_y
            );
            current_quad = 2;
            continue;
        }

        if segment_place > 0.5 && segment_place <= 0.75 {
            if current_quad < 3 {
                debug!("entered quad 3");
                last_x = gen_random_safely(0.0, max_x);
                last_y = gen_random_safely(-max_y, 0.0);
                path_builder.line_to(Vec2::new(last_x, last_y));
            } else {
                last_x = gen_random_safely(0.0, last_x);
                // For quads 1 & 3 using `last_y` seems less necessary in terms of
                // edges crossing each other. Crossing does occur, but much less
                // frequently. However, using it does eliminate all crossing, seemingly
                // at the cost of more "conservative shapes" (but I have not verified it).
                last_y = gen_random_safely(-max_y, last_y);
                // last_y = rng.gen_range(-winsetup.max_y..0.0);
                path_builder.line_to(Vec2::new(last_x, last_y));
            }
            debug!(
                "---i: {}, segment_place: {}, x: {}, y: {}",
                _i, segment_place, last_x, last_y
            );
            current_quad = 3;
            continue;
        }

        if segment_place > 0.75 && segment_place <= 1.0 {
            // Check if it's the first time in this quadrant
            if current_quad < 4 {
                debug!("entered quad 4");
                last_x = gen_random_safely(-max_x, 0.0);
                last_y = gen_random_safely(-max_y, 0.0);
                path_builder.line_to(Vec2::new(last_x, last_y));
            } else {
                last_x = gen_random_safely(-max_x, last_x);
                // In quads 2 & 4, where the horizontal direction of the shape generation
                // changes on the next quadrant, it seems that using `last_y` in the range
                // makes better shapes (edges don't cross)
                //
                // last_y = rng.gen_range(-winsetup.max_y..0.0);
                last_y = gen_random_safely(last_y, 0.0);
                path_builder.line_to(Vec2::new(last_x, last_y));
            }
            debug!(
                "---i: {}, segment_place: {}, x: {}, y: {}",
                _i, segment_place, last_x, last_y
            );
            current_quad = 4;
            continue;
        }
    }
    debug!("--end-{} segments shape---\n", num_segments);
    path_builder.close();

    // Some kind of weirdness with the Path location is preventing me from returning the
    // built shape, so I am returning the builder itself
    // return path_builder.build().0;
    return path_builder;
}
