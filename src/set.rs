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
        match **self {
            Tree::E => Rc::new(Tree::T(Tree::empty(), x, Tree::empty())),
            Tree::T(ref left, ref y, ref right) => {
                if x < *y {
                    Rc::new(Tree::T(left.insert(x), y.clone(), Rc::clone(right)))
                } else if x > *y {
                    Rc::new(Tree::T(Rc::clone(left), y.clone(), right.insert(x)))
                } else {
                    Rc::clone(self)
                }
            },
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
}
