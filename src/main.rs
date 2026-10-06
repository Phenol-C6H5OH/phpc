pub mod commands;
pub mod print_all;
fn main() {
    let mut args=std::env::args();
    args.next(); //跳过程序名
    if let Some(c) = args.next(){
        match c.as_str(){
            "printall"=>commands::print_all(),
            _=>(),
        }
    }
}