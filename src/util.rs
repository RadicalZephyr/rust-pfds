pub struct Iterate<I, F> {
    current: I,
    f: F,
}

impl<I, F> Iterator for Iterate<I, F>
where F: Fn(&I) -> I,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        use std::mem;
        let next = (self.f)(&self.current);
        Some(mem::replace(&mut self.current, next))
    }
}

pub fn iterate<I, F>(init: I, f: F) -> impl Iterator<Item = I>
where F: Fn(&I) -> I,
{
    Iterate { current: init, f}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_inc() {
        let res = iterate(5, |x| x + 1);
        assert_eq!(vec![5, 6, 7, 8], res.take(4).collect::<Vec<u8>>());
    }

    #[test]
    fn iterate_powers_of_two() {
        let res = iterate(1, |x| x * 2);
        assert_eq!(vec![1, 2, 4, 8], res.take(4).collect::<Vec<u8>>());
    }
}
