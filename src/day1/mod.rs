
//use std::env;
//use std::fs;

fn fuel_requirement(mass: i32) -> i32 {
    ((mass as f32 / 3.0).floor() - 2.0) as i32
}

pub fn solve_part_1() -> i32 {

    let input = include_str!("input");

    //println!("This is my current dir: {}", env::current_dir().expect("This should have work"));
    //println!("Reading file {}", file);
    //let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    //

    let sum = input.lines().map(|x| {
        let value = x.parse::<i32>().unwrap();
        fuel_requirement(value)
    }).sum::<i32>();

    println!("The solution to part 1 is: {}", sum);

    sum
}

pub fn solve_part_2() -> i32 {

    let total_mass = solve_part_1();
    let mut total_fuel_required = 0;

    let mut mass = 0;
    loop {
        let fuel_required = fuel_requirement(total_mass);
        if fuel_required > 0 {
            mas += fuel_required;

        } else {
            break;
        }

    }

    println!("The solution to part 2 is: {}", sum);
    sum
}

