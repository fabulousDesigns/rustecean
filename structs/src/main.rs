// struct
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
// tuple struct
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);
// unit-like structures
struct AlwaysEqual;
fn main() {
  let user1 = User {
    active: true,
    username: String::from("user1"),
    email: String::from("user@example.com"),
    sign_in_count: 1,
  };
  let mut user2 = User {
    active: true,
    username: String::from("user2"),
    email: String::from("user2@gmail.com"),
    sign_in_count: 1,
  };
  user2.active = false;
  user2.email = String::from("skadoosh@gmaiil.com");
  // 
  build_user(user2.email, user2.username);
  // tuple structs
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  // unit like structure
  let subject = AlwaysEqual;
  
}

fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username,
    email,
    sign_in_count: 1,
  }
}