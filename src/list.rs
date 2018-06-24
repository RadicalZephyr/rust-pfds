use std::rc::Rc;

#[derive(Debug)]
pub struct IndexOutOfRange;

pub trait Sequence<E: Clone>: Sized {
    fn is_empty(&self) -> bool;

    fn cons(&self, el: E) -> Self;

    fn first(&self) -> Option<&E>;

    fn rest(&self) -> Self;

    fn update(&self, index: usize, val: E) -> Result<Self, IndexOutOfRange>;

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
        match **self {
            List::Nil => true,
            List::Cons(_, _) => false,
        }
    }

    fn cons(&self, el: E) -> Self {
        Rc::new(List::Cons(el, Rc::clone(self)))
    }

    fn first(&self) -> Option<&E> {
        match **self {
            List::Nil => None,
            List::Cons(ref el, _) => Some(el),
        }
    }

    fn rest(&self) -> Self {
        match **self {
            List::Nil => Rc::clone(self),
            List::Cons(_, ref rest) => Rc::clone(rest),
        }
    }

    fn update(&self, index: usize, val: E) -> Result<Self, IndexOutOfRange> {
        if index == 0 {
            Ok(self.rest().cons(val))
        } else {
            match **self {
                List::Nil => Err(IndexOutOfRange),
                List::Cons(ref head, ref rest) => {
                    Ok(rest.update(index - 1, val)?.cons(head.clone()))
                }
            }
        }
    }

    fn concat(&self, other: &Self) -> Self {
        match **self {
            List::Nil => Rc::clone(other),
            List::Cons(ref head, ref rest) => {
                rest.concat(other).cons(head.clone())
            }
        }
    }
}


fn suffixes<E: Clone>(list: &Rc<List<E>>) -> Rc<List<Rc<List<E>>>> {
    if list.is_empty() {
        List::new().cons(Rc::clone(list))
    } else {
        suffixes(&list.rest()).cons(Rc::clone(list))
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

        let new_l = l.update(0, 4).unwrap();
        assert_eq!(new_l.first(), Some(&4));
        assert_eq!(new_l.rest().first(), Some(&1));

        assert!(Rc::ptr_eq(&l.rest(), &new_l.rest()));
    }

    #[test]
    fn update_copies_all_dependent_nodes() {
        let l = List::new().cons(1).cons(2).cons(3);

        let new_l = l.update(1, 4).unwrap();
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

    #[test]
    fn suffixes_of_nil() {
        let l: Rc<List<u8>> = List::new();
        let s = suffixes(&l);

        assert_eq!(s.first(), Some(&l));
    }

    #[test]
    fn suffixes_of_one() {
        let l = List::new().cons(1);
        let s = suffixes(&l);

        assert_eq!(s.first().unwrap().first(), Some(&1));
        assert_eq!(**(s.rest().first().unwrap()), List::Nil);
    }

    #[test]
    fn suffixes_of_many() {
        let l = List::new().cons(1).cons(2);
        let s = suffixes(&l);

        assert_eq!(s.first().unwrap().first(), Some(&2));
        assert_eq!(s.first().unwrap().rest().first(), Some(&1));
        assert_eq!(s.rest().first().unwrap().first(), Some(&1));
        assert_eq!(**(s.rest().rest().first().unwrap()), List::Nil);
    }
}
