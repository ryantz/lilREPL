mod helpers;
use helpers::helper_fn::*;
use helpers::ui_cmpts::*;

#[derive(Debug)]
enum UserType {
    ADMIN,
    USER,
    NOTSELECTED,
}

#[derive(PartialEq)]
enum NavChoice {
    BUILDER,
    VIEW,
    NOTSELECTED,
}

#[derive(Debug)]
struct Profile {
    name: String,
    age: u8,
    user_type: UserType,
}

impl Profile {
    fn show(&self) {
        println!("Your Profile:\n\nName: {}\nAge: {}\nUsertype: {:?}", self.name, self.age, self.user_type);
        //dbg!("{self:#?}");
    }

    fn build(name: String, age: u8, user_type_selection: u8) -> Self {
        let user_type_selected = match user_type_selection {
            1 => UserType::ADMIN,
            2 => UserType::USER,
            _ => UserType::NOTSELECTED,
        };

        Self {
            name,
            age,
            user_type: user_type_selected,
        }
    }
}

fn main() {
    profile_builder_greeting();

    let greeting_choice: NavChoice = navigate(string_to_int(display_then_read("Navigation:\n1. Build profile,\n2. View profile\n\n")));

    if greeting_choice == NavChoice::BUILDER {
        let name = display_then_read("Please enter your name: ");
        let age = string_to_int(display_then_read("Please enter your age: "));
        let selection = string_to_int(display_then_read("Please enter your user type: "));

        let user_profile = Profile::build(name, age, selection);

        insert_line();

        user_profile.show();
    } else {
        not_done_notice();
    }
    insert_line();
}

fn navigate(choice: u8) -> NavChoice {
    let chosen_route = match choice {
        1 => NavChoice::BUILDER,
        2 => NavChoice::VIEW,
        _ => NavChoice::NOTSELECTED,
    };

    chosen_route
}


// TODO:: need to use a vec. 
fn save_profile() {
    todo!()
}
