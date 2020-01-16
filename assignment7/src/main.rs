mod door {
    pub mod feature {
        pub fn open() {
            println!("\nWe are in main.rs -> door -> feature -> open()");
            println!("Open the door");
        }
        pub fn close() {
            println!("\nWe are in main.rs -> door -> feature -> close()");
            println!("Close the door");
        }
    }
}

mod lib;
use portfolio;

fn main() {
    //Question 1
    door::feature::open(); //Calling module fuction using relative path
    crate::door::feature::close(); //Calling module method using absolute path

    //Question 2
    lib::draw::shape::triangle();
    lib::draw::shape::rectangle();

    //Question 3
    portfolio::cv::education::higher_education();
    portfolio::cv::education::matriculation();
}
