use std::rc::Rc;

use tree::Tree;
use util::iterate;

pub struct AlreadyPresent;

pub trait Set<E>
where E: Clone + PartialOrd
{
    fn empty() -> Self;
    fn member(&self, val: &E) -> bool;
    fn insert(&self, val: E) -> Self;
}

#[derive(Clone, Debug)]
struct UnbalancedSet<T>(UnbalancedMap<Rc<T>>);

impl<T> Set<T> for UnbalancedSet<T>
where T: Clone + PartialOrd
{
    fn empty() -> UnbalancedSet<T> {
        UnbalancedSet(UnbalancedMap::empty())
    }

    fn member(&self, x: &T) -> bool {
        self.0.lookup(x).is_some()
    }

    fn insert(&self, val: T) -> UnbalancedSet<T> {
        match self.0.insert(Rc::new(val)) {
            Ok(m) => UnbalancedSet(m),
            Err(AlreadyPresent) => self.clone(),
        }
    }
}

pub trait MapEntry: Clone {
    type Key: PartialOrd;
    type Value;

    fn new(k: Self::Key, v: Self::Value) -> Self;
    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
}

impl<T> MapEntry for Rc<T>
where T: PartialOrd
{
    type Key = T;
    type Value = T;

    fn new(k: Self::Key, _v: Self::Value) -> Self {
        Rc::new(k)
    }

    fn key(&self) -> &Self::Key {
        self.as_ref()
    }

    fn value(&self) -> &Self::Value {
        self.as_ref()
    }
}

impl<K, V> MapEntry for (K, V)
where K: Clone + PartialOrd,
      V: Clone,
{
    type Key = K;
    type Value = V;

    fn new(k: Self::Key, v: Self::Value) -> Self {
        (k, v)
    }

    fn key(&self) -> &Self::Key {
        &(*self).0
    }

    fn value(&self) -> &Self::Value {
        &(*self).1
    }
}

pub trait FiniteMap: Sized {
    type Entry: MapEntry;
    type Key;
    type Value;

    fn empty() -> Self;
    fn insert(&self, e: Self::Entry) -> Result<Self, AlreadyPresent>;
    fn bind(&self, k: Self::Key, v: Self::Value) -> Self;
    fn lookup(&self, k: &Self::Key) -> Option<&Self::Value>;
}

#[derive(Clone, Debug)]
struct UnbalancedMap<T>(Rc<Tree<T>>);

impl<T> FiniteMap for UnbalancedMap<T>
where T: MapEntry,
{
    type Entry = T;
    type Key = T::Key;
    type Value = T::Value;

    fn empty() -> UnbalancedMap<T> {
        UnbalancedMap(Tree::empty())
    }

    fn insert(&self, e: Self::Entry) -> Result<Self, AlreadyPresent> {
        fn iter<T>(t: &Rc<Tree<T>>, x: T, candidate: Option<&T>)
                      -> Result<Rc<Tree<T>>, AlreadyPresent>
        where T: MapEntry,
        {
            match **t {
                Tree::E => {
                    match candidate {
                        Some(c) if c.key() == x.key() => Err(AlreadyPresent),
                        Some(_) | None => {
                            Ok(Tree::leaf(x))
                        }
                    }
                },
                Tree::T(ref left, ref y, ref right) => {
                    if x.key() < y.key() {
                        Ok(Tree::node(&iter(left, x, candidate)?,
                                      (*y).clone(),
                                      right))
                    } else {
                        Ok(Tree::node(left,
                                      (*y).clone(),
                                      &iter(right, x, Some(y))?))
                    }
                }
            }
        }

        iter(&self.0, e, None).map(|t| UnbalancedMap(t))
    }

    fn bind(&self, k: Self::Key, v: Self::Value) -> Self {
        match self.insert(T::new(k, v)) {
            Ok(m) => m,
            Err(AlreadyPresent) => self.clone(),
        }
    }

    fn lookup(&self, k: &Self::Key) -> Option<&Self::Value> {
        fn iter<'a, T>(t: &'a Rc<Tree<T>>, x: &T::Key) -> Option<&'a T::Value>
        where T: MapEntry,
        {
            match **t {
                Tree::E => None,
                Tree::T(ref left, ref y, ref right) => {
                    if x < y.key() {
                        iter(left, x)
                    } else if x > y.key() {
                        iter(right, x)
                    } else {
                        Some(y.value())
                    }
                }
            }
        }

        iter(&self.0, k)
    }
}

fn complete<E>(depth: usize, value: E) -> Rc<Tree<E>>
where E: Clone,
{
    iterate(Tree::leaf(value.clone()),
            |subtree| {
                Tree::node(&subtree, value.clone(), &subtree)
            })
        .skip(depth-1).next().unwrap()
}

fn tree_of<E>(size: usize, value: E) -> Rc<Tree<E>>
where E: Clone
{
    fn subtree_size(x: usize) -> usize {
        ((x - 1) as f64 / 2.0).floor() as usize
    }

    fn create2<E>(m: usize, value: E) -> (Rc<Tree<E>>, Rc<Tree<E>>)
    where E: Clone
    {
        if m == 0 {
            (Tree::leaf(value), Tree::empty())
        } else {
            (tree_of(m+1, value.clone()), tree_of(m, value))
        }
    }

    match size {
        0 => Tree::empty(),
        1 => Tree::leaf(value),
        size if size % 2 == 0 => {
            let (larger, smaller) = create2(subtree_size(size), value.clone());
            Tree::node(&larger, value, &smaller)
        },
        size if size % 2 == 1 => {
            let subtree = tree_of(subtree_size(size), value.clone());
            Tree::node(&subtree, value, &subtree)
        },
        _ => unreachable!("all numbers are odd or even"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tree::BinaryTree;

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
        assert_eq!(1, Rc::strong_count(&(t.0).0));

        let t2 = t.insert(2);
        assert!(t2.member(&1));
        assert!(t2.member(&2));
        assert!(t2.member(&3));
        assert_eq!(2, Rc::strong_count(&(t.0).0));
    }

    #[test]
    fn complete_one() {
        let t = complete(1, ());
        assert_eq!(1, t.count());
    }

    #[test]
    fn complete_two() {
        let t = complete(2, ());
        assert_eq!(3, t.count());
    }

    #[test]
    fn complete_three() {
        let t = complete(3, ());
        assert_eq!(7, t.count());
    }

    #[test]
    fn tree_of_one() {
        let t = tree_of(1, ());
        println!("{:?}", t);
        assert_eq!(1, t.count());
        assert_eq!(1, t.depth());
    }

    #[test]
    fn tree_of_two() {
        let t = tree_of(2, ());
        println!("{:?}", t);
        assert_eq!(2, t.count());
        assert_eq!(2, t.depth());
    }

    #[test]
    fn tree_of_four() {
        let t = tree_of(4, ());
        println!("{:?}", t);
        assert_eq!(4, t.count());
        assert_eq!(3, t.depth());
        assert_eq!(2, t.left().unwrap().depth());
        assert_eq!(1, t.right().unwrap().depth());
    }

    #[test]
    fn tree_of_ten() {
        let t = tree_of(10, ());
        println!("{:?}", t);
        assert_eq!(10, t.count());
        assert_eq!(4, t.depth());
        assert_eq!(3, t.left().unwrap().depth());
        assert_eq!(3, t.right().unwrap().depth());
    }

    //#[test]
    fn display_it() {
        let t = tree_of(5, 1);
        println!("{}", t);
        assert_eq!(1, 2);
    }

    #[test]
    fn map_of_one() {
        let m = UnbalancedMap::<(&str, u8)>::empty().bind("zero", 0u8);
        assert_eq!(Some(&0), m.lookup(&"zero"));
    }

    #[test]
    fn map_of_two() {
        let m = UnbalancedMap::<(&str, u8)>::empty()
            .bind("zero", 0u8)
            .bind("one", 1u8);
        assert_eq!(Some(&0), m.lookup(&"zero"));
        assert_eq!(Some(&1), m.lookup(&"one"));
    }

    #[test]
    fn map_double_bind() {
        let m = UnbalancedMap::<(&str, u8)>::empty()
            .bind("zero", 0u8)
            .bind("one", 1u8);
        let m1 = m.bind("one", 15);
        assert_eq!(Some(&1), m.lookup(&"one"));
    }
}
