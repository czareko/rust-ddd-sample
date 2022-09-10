use fake::{Dummy};

#[derive(Clone, Dummy, PartialEq, Eq,Debug)]
pub enum Gender{
    Female,
    Male,
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