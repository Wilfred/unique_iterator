use std::collections::HashSet;
use std::hash::Hash;

struct UniqueState<I>
    where I: Iterator
{
    seen: HashSet<I::Item>,
    underlying: I,
}

trait Unique: Iterator {
    fn unique(self) -> UniqueState<Self>
        where Self::Item: Hash + Eq + Clone,
              Self: Sized,
    {
        UniqueState { seen: HashSet::new(), underlying: self }
    }
}

impl<I> Unique for I where I: Iterator {}

impl<I> Iterator for UniqueState<I>
    where I: Iterator,
          I::Item: Hash + Eq + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.underlying.next() {
            if !self.seen.contains(&x) {
                self.seen.insert(x.clone());
                return Some(x)
            }
        }
        None
    }
}
#[test]
fn it_works() {
}
