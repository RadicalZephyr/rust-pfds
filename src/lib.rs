pub trait Sequence<E> {
    type R1: Sequence<E>;
    type R2: Sequence<E>;

    fn cons(self, el: E) -> Self::R1;

    fn first(&self) -> Option<&E>;

    fn rest(&self) -> Self::R2;
}

#[derive(Debug, PartialEq)]
enum List<E> {
    Nil,
    Cons(E, Box<List<E>>),
}

impl<E> List<E> {
    pub fn new() -> Self {
        List::Nil
    }
}

impl<E> Sequence<E> for List<E> {
    type R1 = Self;
    type R2 = Self;

    fn cons(self, el: E) -> Self::R1 {
        List::Cons(el, Box::new(self))
    }

    fn first(&self) -> Option<&E> {
        use List::*;

        match *self {
            Nil => None,
            Cons(ref el, _) => Some(el),
        }
    }

    fn rest(&self) -> Self::R2 {
        List::Nil
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_on_nil() {
        let l: List<u8> = List::new();
        assert_eq!(l.first(), None);
    }

    #[test]
    fn first_on_singleton() {
        let l: List<u8> = List::new().cons(1);
        let el = l.first();
        assert_eq!(el, Some(&1));
    }
}
