use macros::AutoDebug;

#[derive(AutoDebug)]
struct BulkString {
    inner: String,
    nothing: (),
    point: (i32, i32),
}

fn main() {
    let bulk_string = BulkString {
        inner: "hello".to_string(),
        nothing: (),
        point: (1, 2),
    };
    println!("{:?}", bulk_string);
}
