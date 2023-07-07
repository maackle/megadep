use alpha::Alpha as Aleph;
use alpha::Beta as BBBBBBBBBBBBB;

pub struct Alpha;

pub struct Beta(Alpha);
pub struct Beeet(Aleph);

pub fn alphas(_: Aleph) -> Alpha {
    Alpha
}

pub fn betas(beta: BBBBBBBBBBBBB) -> Beta {
    Beta(alphas(beta.0))
}
