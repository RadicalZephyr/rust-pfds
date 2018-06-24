use std::{cmp, fmt};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum Tree<E> {
    E,
    T(Rc<Tree<E>>, E, Rc<Tree<E>>),
}

#[derive(Clone, Debug)]
enum Alignment {
    Left, Right
}

impl<E> fmt::Display for Tree<E>
where E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::Alignment::*;

        fn format_value<E>(node: &Tree<E>) -> String
        where E: fmt::Display {
           node.value().map(|v| format!("{}", v)).unwrap_or("( )".to_string())
        }
        let aligns = vec![Left, Right];
        let depth = self.depth();
        let width = f.width().unwrap_or(3);
        let widths = iterate(width, |w| (2*w)+1)
            .skip(1)
            .take(depth-1)
            .collect::<Vec<_>>();
        let width = widths.first().unwrap().clone()+1;
        write!(f, "{:width$}{: ^width$}", "", format_value(self), width=width)?;
        let mut nodes = vec![self.left(), self.right()];
        for width in widths.into_iter().rev() {
            let next_nodes = nodes.iter()
                .flat_map(|n| n.as_ref().map(|n| vec![n.left(), n.right()]).unwrap_or(vec![]))
                .collect();

            write!(f, "\n ")?;
            for i in 0..nodes.len() {
                let edge = if i % 2 == 0 { "/" } else { "\\ " };
                write!(f, " {: ^width$} ", edge, width=width-2)?;
            }
            write!(f, "\n")?;
            let width = cmp::max((width-1)/2, 3);
            for (item, align) in nodes.into_iter().zip(aligns.iter().cycle()) {
                let item = item.unwrap_or(Tree::empty());
                match align {
                    Left => {
                        write!(f, "{:width$}", "", width=(width/2)+1)?;
                        write!(f, "{: ^width$}{:width$}", format_value(item.as_ref()), "", width=width)?;
                    },
                    Right => {
                        write!(f, "{: ^width$}", format_value(item.as_ref()), width=width)?;
                        //write!(f, "{:width$}", "", width=width-1)?;
                    },
                };
            }
            nodes = next_nodes;
        }
        Ok(())
    }
}

impl<E> Tree<E> {
    pub fn empty() -> Rc<Self> {
        Rc::new(Tree::E)
    }

    pub fn leaf(x: E) -> Rc<Self> {
        Rc::new(Tree::T(Tree::empty(), x, Tree::empty()))
    }

    pub fn node(left: &Rc<Self>, x: E, right: &Rc<Self>) -> Rc<Self> {
        Rc::new(Tree::T(Rc::clone(left), x, Rc::clone(right)))
    }
}

pub trait BinaryTree: Sized {
    type Item;

    fn value(&self) -> Option<&Self::Item>;
    fn left(&self)  -> Option<Rc<Self>>;
    fn right(&self) -> Option<Rc<Self>>;
    fn count(&self) -> usize;
    fn depth(&self) -> usize;
}

impl<E> BinaryTree for Tree<E> {
    type Item = E;

    fn value(&self) -> Option<&Self::Item> {
        match self {
            Tree::E => None,
            Tree::T(_, ref value, _) => Some(value),
        }
    }

    fn left(&self) -> Option<Rc<Self>> {
        match self {
            Tree::E => None,
            Tree::T(ref left, _, _) => Some(Rc::clone(left)),
        }
    }

    fn right(&self) -> Option<Rc<Self>> {
        match self {
            Tree::E => None,
            Tree::T(_, _, ref right) => Some(Rc::clone(right)),
        }
    }

    fn count(&self) -> usize {
        match self {
            Tree::E => 0,
            Tree::T(ref left, _, ref right) => 1 + left.count() + right.count(),
        }
    }

    fn depth(&self) -> usize {
        match self {
            Tree::E => 0,
            Tree::T(ref left, _, ref right) => {
                vec![left.depth(), right.depth()]
                    .iter()
                    .max()
                    .unwrap()
                    .clone() + 1
            },
        }
    }
}

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
            let subtree_size = ((size - 1) as f64 / 2.0).floor() as usize;
            let (larger, smaller) = create2(subtree_size, value.clone());
            Tree::node(&larger, value, &smaller)
        },
        size if size % 2 == 1 => {
            let subtree_size = ((size - 1) as f64 / 2.0).floor() as usize;
            let subtree = tree_of(subtree_size, value.clone());
            Tree::node(&subtree, value, &subtree)
        },
        _ => unreachable!("all numbers are odd or even"),
    }
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
        assert_eq!(1, Rc::strong_count(&(t.0).0));

        let t2 = t.insert(2);
        assert!(t2.member(&1));
        assert!(t2.member(&2));
        assert!(t2.member(&3));
        assert_eq!(2, Rc::strong_count(&(t.0).0));
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
