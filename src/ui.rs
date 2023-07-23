use std::io;


pub struct UI;
impl UI{
    pub fn print_menu(){
        println!("1. Simple scan a single port");
        println!("2. Simple scan a range of ports");
        println!("3. Simple scan most common ports");
    }

    pub fn get_input() -> String{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        return input.trim().to_string();
    }


}
