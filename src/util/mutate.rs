pub trait Mutate {
    fn mutate<F>(&mut self, func: F)
    where
        F: FnOnce(&mut Self),
    {
        func(self);
    }
}

impl<T> Mutate for T {}
