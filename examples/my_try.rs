use anyhow::Result;

/// 实现一个Try的宏
#[macro_export]
macro_rules! my_try {
    ($x:expr) => {
        match $x {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }
    };
}

fn main() -> Result<()> {
    let ret = my_try!(second(my_try!(first("hello"))));
    // 这里可以执行，但是同时会panic
    println!("{:?}", ret);

    Ok(())
}

fn first(msg: impl AsRef<str>) -> Result<String> {
    Ok(format!("first:{}", msg.as_ref()))
}
fn second(msg: impl AsRef<str>) -> Result<String> {
    Err(anyhow::anyhow!("second:{}", msg.as_ref()))
}
