use std::{cmp, fmt};
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
           node.value().map(|v| format!("{}", v)).unwrap_or("".to_string())
        }
        let aligns = vec![Left, Right];
        let depth = self.depth();
        let left_depth = self.left().map(|t| t.depth()).unwrap_or_default();
        let width = f.width().unwrap_or(3);
        let widths = iterate(width, |w| 2*w+1)
            .skip(1)
            .take(depth-1)
            .collect::<Vec<_>>()
            .into_iter()
            .rev();

        write!(f, "{: ^width$}", format_value(self), width=width)?;
        let mut nodes = vec![self.left(), self.right()];
        for width in widths {
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
            let next_width = (width-1)/2;
            for (item, align) in nodes.into_iter().zip(aligns.iter().cycle()) {
                match align {
                    Left => {
                        write!(f, "{:width$}", "", width=width)?;
                        write!(f, "{: <width$}{:width$}", format_value(self), "", width=width)?;
                    },
                    Right => {
                        write!(f, "{: >width$}", format_value(self), width=width)?;
                        write!(f, "{:width$}", "", width=width-1)?;
                    },
                };
            }
            nodes = next_nodes;
        }
        let width = 3;
        write!(f, "\n ")?;
        for i in 0..nodes.len() {
            let edge = if i % 2 == 0 { "/" } else { "\\ " };
            write!(f, " {: ^width$} ", edge, width=width-2)?;
        }
        write!(f, "\n")?;
        for (item, align) in nodes.into_iter().zip(aligns.iter().cycle()) {
            match align {
                Left => write!(f, " {: <width$}", format_value(self), width=width)?,
                Right => write!(f, "{: >width$}", format_value(self), width=width)?,
            };
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

fn complete<E>(depth: usize, value: E) -> Rc<Tree<E>>
where E: Clone,
{
    let empty = Tree::empty();
    iterate(Rc::new(Tree::T(Rc::clone(&empty), value.clone(), empty)),
            |subtree| {
                Rc::new(Tree::T(Rc::clone(subtree), value.clone(), Rc::clone(subtree)))
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
            Rc::new(Tree::T(larger, value, smaller))
        },
        size if size % 2 == 1 => {
            let subtree_size = ((size - 1) as f64 / 2.0).floor() as usize;
            let subtree = tree_of(subtree_size, value.clone());
            Rc::new(Tree::T(Rc::clone(&subtree),
                            value,
                            Rc::clone(&subtree)))
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

    #[test]
    fn display_it() {
        let t = tree_of(5, 1);
        println!("{}", t);
        assert_eq!(1, 2);
    }
}
