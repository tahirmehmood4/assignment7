mod it_company {
    pub mod professional_services {
        pub fn print() {
            println!("Welcome to IT_COMPANY::Professional Services Department!");
        }
    }
}
use crate::it_company::professional_services;
use crate::it_company::professional_services::print; //idiomatic path NOT PREFERRED!
fn main() {
    print!("Display Message Called by Absolute Path!  : ");
    professional_services::print(); //absolute path
    print!("Display Message Called by Relative Path!  : ");
    it_company::professional_services::print(); //relative path
    print!("Display Message Called by Idiomatic Path! : ");
    print(); //idiomatic path NOT PREFERRED!
}
