extern crate easy_color;
use easy_color::*;

fn main() {
    
    print!("{}","Hello.world".blue().red().on_blue().italic());
    
    println!("{}", "Hello World!".green().red().on_blue().bold().underline().on_yellow().italic());
    
    print!("{}","xx".red());

    let xx = "xx".bold();
    print!("{:?}",xx);
}