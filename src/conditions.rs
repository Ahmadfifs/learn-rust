pub fn run(){
	// if condition
	if 1 < 2 {
		println!("hello world");
	}

	// infinite loop
	let mut a = 1;
	loop {
		if a == 4 {
			break;
		} else {
			println!("loop: {a}");
			a = a + 1;
		}
	}

	// while loop
	let mut b = 1;
	while b < 4 && true {
		println!("while loop: {b}");
		b = b + 1;
		
	}
}