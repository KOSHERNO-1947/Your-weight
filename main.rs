use std::{thread, time, io::Write};
fn moon(i: f32) -> f32 {
	i*0.166
}
fn mars(i: f32) -> f32 {
	i*0.4
}
fn mercury(i: f32) -> f32 {
	i*0.38
}
fn venus(i: f32) -> f32 {
	i*0.9
}
fn print(st: &str) {
	for i in 0..st.len() {
		print!("{}", st.chars().nth(i).unwrap());
		std::io::stdout().flush().unwrap();
		thread::sleep(time::Duration::from_millis(100));
	}
	print!("\n");
	return;
}
fn main() {
	print("Hello, it is a program to calculate your weight on different places");
	print("So, what is your weight?");
	let mut weight = String::new();
	std::io::stdin().read_line(&mut weight);
	print("Now choose place: (1) moon, (2) mars, (3) mercury or (4) venus");
	let mut number_str = String::new();
	std::io::stdin().read_line(&mut number_str);
	let number: i32 = number_str.trim().parse::<i32>().unwrap();
	print("Calculating...........................");
	if number == 1 {println!("{}", moon(weight.trim().parse::<f32>().unwrap()));}
	if number == 2 {println!("{}", mars(weight.trim().parse::<f32>().unwrap()));}
	if number == 3 {println!("{}", mercury(weight.trim().parse::<f32>().unwrap()));}
	if number == 4 {println!("{}", venus(weight.trim().parse::<f32>().unwrap()));}
}	
