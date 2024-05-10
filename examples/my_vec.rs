use macros::my_vec;

fn main() {
    let v = my_vec![1, 2, 3];
    println!("{:?}", v);
    let v1: Vec<i32> = my_vec![];
    println!("{:?}", v1);

    let v2 = my_vec![1;3];
    println!("{:?}", v2);
    let v3 = my_vec![1, 2, 3, 4, 5,];
    println!("{:?}", v3)
}
