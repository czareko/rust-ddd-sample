use std::fmt::{Display, Formatter};
use fake::{Dummy};

#[derive(Clone, Copy, Dummy, PartialEq, Eq, Debug)]
pub enum Gender {
    Female,
    Male,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Gender::Female => write!(f, "female"),
            Gender::Male => write!(f, "male"),
        }
    }
}
/*
impl Dummy<Faker> for Gender{

    //Just to mockup

    fn dummy(config: &Faker) -> Self {
        Self::Female
    }

    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        Self::Male
    }
}*/