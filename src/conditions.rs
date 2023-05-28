pub fn run(){
	// if condition
	if 1 < 2 {
		println!("hello world");
	}


	let x = 2;
	match x {
		1 => println!("hello 1"),
		2 => println!("hello 2"),
		_ => println!("other"),
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


	// for loop 
	for n in 0..100 { // from 0 to 99
		print!("{n}");
	}

	println!("");

	for n in 0..=100 { // from 0 to 100
		print!("{n}");
	}
	

}