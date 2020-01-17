mod lib;
fn main() {
    lib::it_company::professional_services::print();
    print!("Print Below Message Using Absolute Path!  - ");
    crate::lib::it_company::professional_services::print(); //Absolute Path
}