use macros::AutoDeref;

#[allow(unused)]
#[derive(Debug, AutoDeref)]
#[auto_deref(field = "inner")]
struct BulkString {
    inner: String,
    nothing: (),
}

fn main() {}
