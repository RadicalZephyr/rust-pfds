use std::{cmp, fmt};
use std::rc::Rc;

use util::iterate;

#[derive(Debug, PartialEq)]
pub enum Tree<E> {
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
