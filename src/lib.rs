use std::rc::Rc;

pub trait Sequence<E: Clone> {
    type R1: Sequence<E>;
    type R2: Sequence<E>;
    type R3: Sequence<E>;

    fn cons(&self, el: E) -> Self::R1;

    fn first(&self) -> Option<&E>;

    fn rest(&self) -> Self::R2;

    fn update(&self, index: u8, val: E) -> Self::R3;
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

impl<E: Clone> Sequence<E> for Rc<List<E>> {
    type R1 = Rc<List<E>>;
    type R2 = Rc<List<E>>;
    type R3 = Rc<List<E>>;

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
        use List::*;
        match **self {
            Nil => self.clone(),
            Cons(_, ref rest) => rest.clone(),
        }
    }

    fn update(&self, index: u8, val: E) -> Self::R3 {
        use List::*;
        if index == 0 {
            Rc::new(Cons(val, self.rest()))
        } else {
            match **self {
                Nil => panic!("ahhhhhh!"),
                Cons(ref el, ref rest) => Rc::new(Cons(el.clone(), rest.update(index - 1, val))),
            }
        }
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

    #[test]
    fn rest_on_nil_is_nil() {
        let l: Rc<List<u8>> = List::new();
        assert_eq!(*(l.rest()), List::Nil);
    }

    #[test]
    fn rest_on_list_drops_first_item() {
        let l = List::new().cons(1).cons(2);
        let rest = l.rest();
        assert_eq!(rest.first(), Some(&1));
    }

    #[test]
    fn update_copies_changed_node() {
        let l = List::new().cons(1).cons(2);
        assert_eq!(l.first(), Some(&2));
        assert_eq!(l.rest().first(), Some(&1));

        let new_l = l.update(0, 4);
        assert_eq!(new_l.first(), Some(&4));
        assert_eq!(new_l.rest().first(), Some(&1));

        assert!(Rc::ptr_eq(&l.rest(), &new_l.rest()));
    }

    #[test]
    fn update_copies_all_dependent_nodes() {
        let l = List::new().cons(1).cons(2).cons(3);

        let new_l = l.update(1, 4);
        assert_eq!(new_l.first(), Some(&3));
        assert_eq!(new_l.rest().first(), Some(&4));
        assert_eq!(new_l.rest().rest().first(), Some(&1));

        assert!(Rc::ptr_eq(&l.rest().rest(), &new_l.rest().rest()));
    }
}
