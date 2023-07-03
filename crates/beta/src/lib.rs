extern crate alpha;

use alpha::Alpha;

pub struct Beta(Alpha);

fn something(alpha: Alpha) -> Beta {
    Beta(alpha)
}
