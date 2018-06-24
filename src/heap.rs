use std::rc::Rc;
use tree::Tree;

pub trait Heap {
    type Item;

    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn insert(&self, x: Self::Item) -> Self;
    fn merge(&self, other: &Self) -> Self;

    fn find_min(&self) -> Option<&Self::Item>;
    fn delete_min(&self) -> Self;
}

type HeapTree<T> = Rc<Tree<(usize, T)>>;

#[derive(Clone, Debug)]
pub struct LeftistHeap<T>(HeapTree<T>);

fn rank<T>(node: &HeapTree<T>) -> usize {
    match **node {
        Tree::E => 0,
        Tree::T(_, (r, _), _) => r,
    }
}

fn make_t<T>(x: &T, a: &HeapTree<T>, b: &HeapTree<T>) -> HeapTree<T>
where T: Clone
{
    if rank(a) >= rank(b) {
        Tree::node(a, (rank(b)+1, (*x).clone()), b)
    } else {
        Tree::node(b, (rank(a)+1, (*x).clone()), a)
    }
}

impl<T> Heap for LeftistHeap<T>
where T: Clone + PartialOrd,
{
    type Item = T;

    fn empty() -> Self {
        LeftistHeap(Tree::<(usize, T)>::empty())
    }

    fn is_empty(&self) -> bool {
        match *self.0 {
            Tree::E => true,
            Tree::T(..) => false,
        }
    }

    fn insert(&self, x: Self::Item) -> Self {
        self.merge(&LeftistHeap(Tree::leaf((1, x))))
    }

    fn merge(&self, other: &Self) -> Self {
        fn iter<T>(h1: &HeapTree<T>, h2: &HeapTree<T>) -> HeapTree<T>
        where T: Clone + PartialOrd,
        {
            match (h1.as_ref(), h2.as_ref()) {
                (Tree::E, _) => Rc::clone(&h2),
                (_, Tree::E) => Rc::clone(&h1),
                (Tree::T(ref a1, (_, ref x), ref b1),
                 Tree::T(ref a2, (_, ref y), ref b2)) => {
                    if *x <= *y {
                        make_t(x, a1, &iter(b1, h2))
                    } else {
                        make_t(y, a2, &iter(h1, b2))
                    }
                }
            }
        }
        LeftistHeap(iter(&self.0, &other.0))
    }

    fn find_min(&self) -> Option<&Self::Item> {
        match *self.0 {
            Tree::E => None,
            Tree::T(_, (_, ref x), _) => Some(x),
        }
    }

    fn delete_min(&self) -> Self {
        match *self.0 {
            Tree::E => self.clone(),
            Tree::T(ref a, _, ref b) => LeftistHeap(Rc::clone(&a)).merge(&LeftistHeap(Rc::clone(b))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tree::BinaryTree;

    #[test]
    fn empty_leftist_heap() {
        let h = LeftistHeap::<&'static str>::empty();
        assert!(h.is_empty());
    }

    #[test]
    fn heap_of_one() {
        let h = LeftistHeap::<u8>::empty().insert(1);
        assert_eq!(Some(&1), h.find_min());
    }

    #[test]
    fn heap_of_two() {
        let h = LeftistHeap::<u8>::empty()
            .insert(3)
            .insert(2);
        assert_eq!(Some(&2), h.find_min());
    }

    #[test]
    fn deleting_from_heap() {
        let h = LeftistHeap::<u8>::empty()
            .insert(5)
            .insert(7);
        assert_eq!(Some(&5), h.find_min());
        let h1 = h.delete_min();
        assert_eq!(Some(&7), h1.find_min());
    }
}
