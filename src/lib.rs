pub trait Sequence<E> {
    type R1: Sequence<E>;
    type R2: Sequence<E>;
    type R3: Sequence<E>;

    fn cons(&self, el: E) -> Self::R1;

    fn first(&self) -> (E, Self::R2);

    fn rest(&self) -> Self::R3;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
