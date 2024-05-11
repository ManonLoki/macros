use macros::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(i32),
    Down,
    Left(DirectionLeft<T>),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionLeft<T> {
    speed: T,
}

impl<T> DirectionLeft<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

fn main() {
    let left: Direction<_> = DirectionLeft::new(42).into();
    let up: Direction<i32> = 10_i32.into();

    println!("Direction:\n up:{:#?}", up);
    println!("Direction:\n left:{:#?}", left);
}
