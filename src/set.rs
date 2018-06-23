use std::rc::Rc;

pub trait Set<E>
where E: Clone + PartialOrd
{
    fn member(&self, val: &E) -> bool;
    fn insert(&self, val: E) -> Self;
}

#[derive(Debug, PartialEq)]
enum Tree<E> {
    E,
    T(Rc<Tree<E>>, E, Rc<Tree<E>>),
}

impl<E> Tree<E> {
    pub fn empty() -> Rc<Self> {
        Rc::new(Tree::E)
    }
}

struct AlreadyPresent;

impl<E> Set<E> for Rc<Tree<E>>
where E: Clone + PartialOrd
{
    fn member(&self, x: &E) -> bool {
        match **self {
            Tree::E => false,
            Tree::T(ref left, ref y, ref right) => {
                if *x < *y {
                    left.member(x)
                } else if *x > *y {
                    right.member(x)
                } else {
                    true
                }
            },
        }
    }

    fn insert(&self, x: E) -> Rc<Tree<E>> {
        fn iter<E>(t: &Rc<Tree<E>>, x: E, candidate: Option<&E>)
                   -> Result<Rc<Tree<E>>, AlreadyPresent>
        where E: Clone + PartialOrd
        {
            match **t {
                Tree::E => {
                    match candidate {
                        Some(c) if *c == x => Err(AlreadyPresent),
                        Some(_) | None => {
                            Ok(Rc::new(Tree::T(Tree::empty(),
                                               x,
                                               Tree::empty())))
                        }
                    }
                },
                Tree::T(ref left, ref y, ref right) => {
                    if x < *y {
                        Ok(Rc::new(Tree::T(iter(left, x, candidate)?,
                                           y.clone(),
                                           Rc::clone(right))))
                    } else {
                        Ok(Rc::new(Tree::T(Rc::clone(left),
                                           y.clone(),
                                           iter(right, x, Some(y))?)))
                    }
                },
            }
        }

        match iter(self, x, None) {
            Ok(new_t) => new_t,
            Err(AlreadyPresent) => Rc::clone(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let t = Tree::<u8>::empty();
        assert!(!t.member(&0));
    }

    #[test]
    fn insert_one() {
        let t = Tree::<u8>::empty().insert(1);
        assert!(t.member(&1));
    }

    #[test]
    fn insert_several() {
        let t = Tree::empty().insert(1).insert(3);
        assert!(t.member(&1));
        assert!(t.member(&3));
    }

    #[test]
    fn insert_many() {
        let t = Tree::empty().insert(2).insert(1).insert(3);
        assert!(t.member(&1));
        assert!(t.member(&2));
        assert!(t.member(&3));
        assert_eq!(Rc::strong_count(&t), 1);

        let t2 = t.insert(2);
        assert!(t2.member(&1));
        assert!(t2.member(&2));
        assert!(t2.member(&3));
        assert_eq!(Rc::strong_count(&t), 2);
    }
}
