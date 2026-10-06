use crossterm::style::{Attribute::Bold,Color,Stylize,style};

/*
antecedent:先行词,表示"Finishing"等词汇
content:后续的内容
*/

///输出成功日志
pub fn log(antecedent:&str,content:&str){
    println!("{} {content}",
        style(antecedent)
        .with(Color::Green)
        .attribute(Bold));
}

///输出失败或错误日志
pub fn elog(antecedent:&str,content:&str){
    eprintln!("{} {content}",
        style(antecedent)
        .with(Color::Red)
        .attribute(Bold));
}

#[cfg(test)]
mod tests{
    #[test]
    ///用默认信息
    fn test_log(){
        crate::log("Testing","log message");
        crate::elog("Testing","error message");
    }
    
    
}