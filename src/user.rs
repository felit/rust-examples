use std::fmt;
#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool
}
impl User {
	pub fn print_user(&self){
	  println!("username:{}\n",self.username);
	}

}
impl fmt::Display for User{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "username为{}，email为{}\n", self.username,self.email)
      }
}
fn main(){
	let mut user1 = User {
	  email: String::from("someone@example.com"),
	  username: String::from("someusername123"),
	  active: true,
	  sign_in_count: 1
	};
	user1.email = String::from("anotheremail@example.com");
	println!("user:{:?}\n",user1);
}