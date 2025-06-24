#[derive(Debug)]
struct Chars(u8, u8, u8,u8, u8);



impl Chars{
	fn new()-> Chars{
		Chars(104, 101, 108, 108, 111)
	}

	fn human_regards(self) -> String{
	
		let mut char_series:Vec::<char> = vec![];
		
		for i in [self.0, self.1, self.2, self.3, self.4].into_iter(){
			char_series.push(i as char);
			println!("{i}");
		}
	char_series.iter().collect::<String>()
	// char_series
		
	}
	
}
pub fn tell(){
	let hello = Chars::new().human_regards();
	println!("{:?}", hello);
}
