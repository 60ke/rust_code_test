use cfg_if::cfg_if;
use macro_test_derive::macro_test_derive;

#[macro_test_derive(K)]
fn hello(){
    unimplemented!()
}

macro_rules! reprint {
    ($e:expr,$d:expr) => {
        cfg_if!{
            if #[cfg(feature = "test")]{
                println!($e,$d)
            }
        }
    };
}


fn main(){
    hello1();
    reprint!("{}",3);

}