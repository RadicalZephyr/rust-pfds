use std::rc::Rc;

pub trait Sequence<E> {
    type R1: Sequence<E>;
    type R2: Sequence<E>;

    fn cons(&self, el: E) -> Self::R1;

    fn first(&self) -> Option<&E>;

    fn rest(&self) -> Self::R2;
}

#[derive(Debug, PartialEq)]
enum List<E> {
    Nil,
    Cons(E, Rc<List<E>>),
}

impl<E> List<E> {
    pub fn new() -> Rc<Self> {
        Rc::new(List::Nil)
    }
}

impl<E> Sequence<E> for Rc<List<E>> {
    type R1 = Rc<List<E>>;
    type R2 = Rc<List<E>>;

    fn cons(&self, el: E) -> Self::R1 {
        Rc::new(List::Cons(el, self.clone()))
    }

    fn first(&self) -> Option<&E> {
        use List::*;
        match **self {
            Nil => None,
            Cons(ref el, _) => Some(el),
        }
    }

    fn rest(&self) -> Self::R2 {
        Rc::new(List::Nil)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_on_nil() {
        let l: Rc<List<u8>> = List::new();
        assert_eq!(l.first(), None);
    }

    #[test]
    fn first_on_singleton() {
        let l = List::new().cons(1);
        let el = l.first();
        assert_eq!(el, Some(&1));
    }

    #[test]
    fn does_not_move_on_cons() {
        let l = List::new();
        let l2 = l.cons(1);
        assert_eq!(*l, List::Nil);
        assert_eq!(l2.first(), Some(&1));
    }
}
