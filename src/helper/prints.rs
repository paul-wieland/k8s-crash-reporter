

pub fn print_welcome_message(){
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");
    println!("******************************************");
    println!("** Welcome to {} v{} **", NAME, VERSION);
    println!("******************************************");
}