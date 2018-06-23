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

struct UnbalancedSet<E>(Rc<Tree<E>>);

impl<E> UnbalancedSet<E> {
    pub fn empty() -> UnbalancedSet<E> {
        UnbalancedSet(Tree::empty())
    }
}

impl<E> Set<E> for UnbalancedSet<E>
where E: Clone + PartialOrd
{
    fn member(&self, x: &E) -> bool {
        fn iter<E>(t: &Rc<Tree<E>>, x: &E) -> bool
        where E: Clone + PartialOrd,
        {
            match **t {
                Tree::E => false,
                Tree::T(ref left, ref y, ref right) => {
                    if *x < *y {
                        iter(left, x)
                    } else if *x > *y {
                        iter(right, x)
                    } else {
                        true
                    }
                },
            }
        }

        iter(&self.0, x)
    }

    fn insert(&self, x: E) -> UnbalancedSet<E> {
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

        match iter(&self.0, x, None) {
            Ok(new_t) => UnbalancedSet(new_t),
            Err(AlreadyPresent) => UnbalancedSet(Rc::clone(&self.0)),
        }
    }
}

struct Iterate<I, F> {
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

fn iterate<I, F>(init: I, f: F) -> impl Iterator<Item = I>
where F: Fn(&I) -> I,
{
    Iterate { current: init, f}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let t = UnbalancedSet::<u8>::empty();
        assert!(!t.member(&0));
    }

    #[test]
    fn insert_one() {
        let t = UnbalancedSet::<u8>::empty().insert(1);
        assert!(t.member(&1));
    }

    #[test]
    fn insert_several() {
        let t = UnbalancedSet::empty().insert(1).insert(3);
        assert!(t.member(&1));
        assert!(t.member(&3));
    }

    #[test]
    fn insert_many() {
        let t = UnbalancedSet::empty().insert(2).insert(1).insert(3);
        assert!(t.member(&1));
        assert!(t.member(&2));
        assert!(t.member(&3));
        assert_eq!(Rc::strong_count(&t.0), 1);

        let t2 = t.insert(2);
        assert!(t2.member(&1));
        assert!(t2.member(&2));
        assert!(t2.member(&3));
        assert_eq!(Rc::strong_count(&t.0), 2);
    }

    #[test]
    fn iterate_inc() {
        let mut res = iterate(5, |x| x + 1);
        assert_eq!(vec![5, 6, 7, 8], res.take(4).collect::<Vec<u8>>());
    }

    #[test]
    fn iterate_powers_of_two() {
        let mut res = iterate(1, |x| x * 2);
        assert_eq!(vec![1, 2, 4, 8], res.take(4).collect::<Vec<u8>>());
    }
}
