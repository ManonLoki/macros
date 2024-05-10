use anyhow::Result;
use macros::my_try;
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
