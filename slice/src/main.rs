struct TextAnalyzer {
    word: String,
}
impl TextAnalyzer {

}

fn main() {
    let v = vec![0,1,2,3,4,5,6,7,8,9,10];
    let s1 = &v[1..3];
    // println!("s1 = {:?}", s1);

    let message = String::from("hello world");
    let hello = &message[0..5];
    let world = &message[6..];
    // println!("hello = {:?}", hello);
    // println!("world = {:?}", world);

    let mut colors = vec!["red", "green", "blue"];
    let slice = &mut colors[1..];
    slice[0] = "purple";
    // println!("slice = {:?}", slice);
    // println!("slice = {:?}", colors);
}
