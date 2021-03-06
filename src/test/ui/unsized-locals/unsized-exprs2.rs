#![feature(unsized_tuple_coercion, unsized_locals)]

struct A<X: ?Sized>(X);

fn udrop<T: ?Sized>(_x: T) {}
fn foo() -> Box<[u8]> {
    Box::new(*b"foo")
}
fn tfoo() -> Box<(i32, [u8])> {
    Box::new((42, *b"foo"))
}
fn afoo() -> Box<A<[u8]>> {
    Box::new(A(*b"foo"))
}

impl std::ops::Add<i32> for A<[u8]> {
    type Output = ();
    fn add(self, _rhs: i32) -> Self::Output {}
}

fn main() {
    udrop::<[u8]>(foo()[..]);
    //~^ERROR cannot move out of indexed content
    // FIXME: should be error
    udrop::<A<[u8]>>(A(*foo()));
}
