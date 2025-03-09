use std::io::{self, Write};

#[derive(Debug)]
struct Profile {
    name: String,
    age: u32,
    weight: u32,
    area: AreaCode,
}

struct CPFcalculation {
    salary: f32,
    percentage: f32,
}

impl CPFcalculation {
    fn calculate_cpf(&self) -> f32 {
        self.salary * self.percentage
    }
}

// tuple structs with no named fields
#[derive(Debug)] struct AreaCode(u32,u32,u32);

fn main() {
    let _boolean: bool = true;

    println!("Welcome user");
    println!("==================");
    
    let name = String::from("Ryan Tan");
    let age = 28;
    let weight =  80;
    let area = AreaCode(21,32,3);

    let name2 = String::from("Someone else");
    let area2 = AreaCode(1,20,33);

    let profile = set_profile(name, age, weight, area);

    
    let profile2 = Profile {
        name : name2,
        area: area2,
        ..profile
    };

    let salary = CPFcalculation {
        salary: 1000.0,
        percentage: 0.8,
    };


    loop {

        print!("lilREPL >> ");
        io::stdout().flush().expect("Failed to flush output that is tagged to print! statement");

        let mut input = String::from("input recieved -> ");

        repl_reply(&mut input);

        println!("{}", input.trim());

        println!("Your profile: ");
        println!("name: {}, age: {}, weight: {}, area1: {}, area1: {}, area1: {}", profile.name, profile.age, profile.weight, profile.area.0, profile.area.1, profile.area.2);
        
        println!("==============================================================");

        println!("Another profile: ");
        println!("{profile2:?}");
        println!("{profile2:#?}"); // println! takes reference
        dbg!(&profile2); //dbg takes ownership
        println!("Salary after CPF is : {}", salary.calculate_cpf());
    }
}

// pass in thru ref
fn repl_reply(input_ref: &mut String) -> &str {
    io::stdin().read_line(input_ref).expect("failed to read input");

    input_ref
    
}

fn set_profile(name: String, age: u32, weight: u32, area: AreaCode) -> Profile {
    let user_prof = Profile {
        name: String::from(name),
        age,
        weight,
        area,
    };

    user_prof
}

