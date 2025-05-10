use std::fs::File;
struct Resource(&'static str);
impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping Resource {}", self.0);
    }
}

fn main() {
    // panic!("Crash and burn!");
    // // panicking in response to a Bug
    // let v = vec![1,2,3];
    // v[99];
    // let _a = Resource("Hello");
    // let _b = Resource("Hello");
    // 
    // println!("About to exit");
    // panic!("Something went wrong!");
    let f = File::open("file.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
