/// 实现一个宏，用于创建一个 Vec
#[macro_export]
macro_rules! my_vec {
    // 捕获没有表达式的情况
    ()=>{
        {
            Vec::new()
        }
    };
    // 捕获 my_vec![elem;n]的情况
    ($elem:expr;$n:expr)=>{
        {
            std::vec::from_elem($elem, $n)
        }
    };
    // 捕获0个或多个表达式 最后的$(,)是为了兼容最后一个表达式后面的逗号
    // $($变量)* 在参数部分标识捕获， 在代码部分标识展开
    // 分隔符貌似只支持,和;
    ($($x:expr),+ $(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}

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

// my_ready! => Pool:Ready / Pool::Pending
#[macro_export]
macro_rules! my_ready {
    ($x:expr) => {
        match $x {
            std::task::Poll::Ready(v) => std::task::Poll::Ready(v),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
