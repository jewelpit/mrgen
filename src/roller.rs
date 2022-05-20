use rand::prelude::SliceRandom;

pub trait Rollable {
    type Item;
    fn roll(&self) -> &Self::Item;
}

impl<T> Rollable for [T] {
    type Item = T;

    fn roll(&self) -> &Self::Item {
        self.choose(&mut rand::thread_rng()).unwrap()
    }
}
