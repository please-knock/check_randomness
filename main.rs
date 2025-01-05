use rand::Rng;
use std::time::Instant;
use std::io;

fn main() { loop {
	println!("\n==========================================================================");
    println!("\n| Please select: \n|\n| 1: Check randomness\n| 2: Highest counter (Need manual abort)\n| 3: Print counter (Need manual abort)\n| 4: Explanation\n");

    let mut selected = String::new();

    io::stdin()
        .read_line(&mut selected)
        .expect("Failed to read line");
        
	let selected: i32 = selected.trim().parse().expect("Please enter a valid number");

	if selected == 1 {
		println!("\n\nPlease wait... this process may take some time.");
		let start = Instant::now();
		let result = random_counter();
		let duration = start.elapsed();
		
		println!("\nCalculation took: {:?}", duration);
		println!("The average of attempts to reach the random number is: {}", result);
		
		//This can almost never happen if the samples are high (like million)
		if result > 101.0 || result < 99.0 {
		println!("\nThe result isn't quite close to 100. Check the number of samples in the calculation() function. Recommended value is a million. Or maybe there could be a problem in the randomness...?");
		}
	}
	
	else if selected == 2 {
		highest_counter();
	}
	
	else if selected == 3 { loop {
            let result_3 = randomness_print();
            println!("{result_3}");
        } }
	
	else if selected == 4 {
		println!("\nThis is a simple code for checking randomness. \n\n[Option 1] The random counter uses the randomness() function to first generate a random number, and then generates a second random number which is inside a loop with a counter. Endlessly comparing two random numbers until they match each other will give a third random number. Then the random_counter() function will loop randomness() a million times, and then add those all, and then divide it by a million to get the average. The average will always be close to 100 because that is just the way how it works. I haven't yet came up with a clear understanding of this code too. However you can somewhat prove it by executing the second function. \n\n[Option 2] Function highest_counter() will execute randomness() inside a loop and only print the results (which is the number of endless tries until the first generated random number matches the second generated random number) that are higher than the previous. You will also see how much time has passed since the execution. It takes less than one second to reach 1000, while from 1000 it takes significant amount of time. Theoretically, there is no end to this calculation. In other words, there is a possibility that first generated random number and second generated random number never becomes identical. \n\n[Option 3] This will loop randomness() and print the outputs without doing any other operations with it.\n");
	}
	
	else {
        println!("Invalid selection. Please select in 1, 2, 3, or 4.\n\n\n");
    }
    
} }

fn random_counter() -> f64 { 
    let mut randomness_sum = 0;

    for _ in 0..1000000 {
        randomness_sum += randomness();
    }

    let average: f64 = randomness_sum as f64 / 1000000.0;
    average
}

fn highest_counter() {
    let mut previous_result: Option<u32> = None;	
    let start_time = Instant::now();

    loop {
        let elapsed_time = start_time.elapsed();
        let seconds = elapsed_time.as_secs();
        let result = randomness();
        if previous_result.is_none() || result > previous_result.unwrap() {
            println!("{:?} seconds, highest: {}", seconds, result);
            previous_result = Some(result);
        }
    }
}

fn randomness() -> u32 {
    let random_number = rand::thread_rng().gen_range(1..=100);
    let mut counter = 1;
    while rand::thread_rng().gen_range(1..=100) != random_number { 
        counter += 1;
    }
    counter
}

fn randomness_print() -> u32 {
    let random_number_3 = rand::thread_rng().gen_range(1..=100);
    let mut counter_3 = 1;
    while rand::thread_rng().gen_range(1..=100) != random_number_3 { 
        counter_3 += 1;
    }
    counter_3
}
