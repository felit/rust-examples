
fn main() {
    println!("Hello, world!");
    println!("world best");
    var();
    tuple_example();
    hello::print_hello();
}
fn var() -> i32 {
		
	let x = 5;
	// let x: int = 5;
	println!("x:{}",x);
	return x;
}
fn tuple_example(){
	let x = (1,"hello");
	let (h,i,j)=(1,2,3);
	println!("h is {}",h);
}
mod hello {
	pub fn print_hello(){
		println!("Hello world!");
	}
}