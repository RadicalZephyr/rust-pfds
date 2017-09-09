use std::rc::Rc;

pub trait Sequence<E: Clone> {
    fn is_empty(&self) -> bool;

    fn cons(&self, el: E) -> Self;

    fn first(&self) -> Option<&E>;

    fn rest(&self) -> Self;

    fn update(&self, index: u8, val: E) -> Self;

    fn concat(&self, other: &Self) -> Self;
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
    fn is_empty(&self) -> bool {
        use List::*;
        match **self {
            Nil => true,
            Cons(_, _) => false,
        }
    }

    fn cons(&self, el: E) -> Self {
        Rc::new(List::Cons(el, self.clone()))
    }

    fn first(&self) -> Option<&E> {
        use List::*;
        match **self {
            Nil => None,
            Cons(ref el, _) => Some(el),
        }
    }

    fn rest(&self) -> Self {
        use List::*;
        match **self {
            Nil => self.clone(),
            Cons(_, ref rest) => rest.clone(),
        }
    }

    fn update(&self, index: u8, val: E) -> Self {
        if index == 0 {
            self.rest().cons(val)
        } else {
            if self.is_empty() {
                panic!("ahhhhhh!")
            } else {
                self.rest().update(index - 1, val).cons(
                    self.first()
                        .unwrap()
                        .clone(),
                )
            }
        }
    }

    fn concat(&self, other: &Self) -> Self {
        if self.is_empty() {
            other.clone()
        } else {
            self.rest().concat(other).cons(
                self.first().unwrap().clone(),
            )
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

    #[test]
    fn concat_joins_two_lists() {
        let xs = List::new().cons(1).cons(2);
        let ys = List::new().cons(3).cons(4);

        let zs = ys.concat(&xs);

        assert_eq!(zs.first(), Some(&4));
        assert_eq!(zs.rest().first(), Some(&3));
        assert_eq!(zs.rest().rest().first(), Some(&2));
        assert_eq!(zs.rest().rest().rest().first(), Some(&1));
    }

    #[test]
    fn concat_leaves_both_lists_usable() {
        let xs = List::new().cons(1).cons(2);
        let ys = List::new().cons(3).cons(4);

        let _zs = ys.concat(&xs);
        assert_eq!(xs.first(), Some(&2));
        assert_eq!(xs.rest().first(), Some(&1));

        assert_eq!(ys.first(), Some(&4));
        assert_eq!(ys.rest().first(), Some(&3));

    }
}
