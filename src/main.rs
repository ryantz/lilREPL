mod helpers;
use helpers::*;

#[derive(PartialEq)]
enum NavChoice {
    BUILDER,
    VIEW,
    NOTSELECTED,
}

fn main() {
    let mut profile_storage: Vec<structs_enums::Profile> = Vec::new();
    ui_cmpts::profile_builder_greeting();

    loop {

        let greeting_choice: NavChoice = navigate(helper_fn::string_to_int(helper_fn::display_then_read("Navigation:\n1. Build profile,\n2. View profile\n\n")));

        if greeting_choice == NavChoice::BUILDER {
            let name = helper_fn::display_then_read("Please enter your name: ");
            let age = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your age: "));
            let selection = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your user type: "));

            let user_profile = structs_enums::Profile::build(name, age, selection);

            ui_cmpts::insert_line();

            //user_profile.show();
            save_profile(user_profile, &mut profile_storage);

        } else if greeting_choice == NavChoice::VIEW {
            view_stored_profiles(&profile_storage);           
        } else {
            ui_cmpts::not_done_notice();
        }
    }
}

fn navigate(choice: u8) -> NavChoice {
    let chosen_route = match choice {
        1 => NavChoice::BUILDER,
        2 => NavChoice::VIEW,
        _ => NavChoice::NOTSELECTED,
    };

    chosen_route
}


fn save_profile(profile_to_save: structs_enums::Profile,  profile_storage:&mut Vec<structs_enums::Profile>) -> &mut Vec<structs_enums::Profile>{
    profile_storage.push(profile_to_save);
    
    profile_storage
}

fn view_stored_profiles(profile_storage: &Vec<structs_enums::Profile>) {
    for element in profile_storage {
        ui_cmpts::insert_line();
        println!("\nname: {}\nage: {}\nuser type:{:?}\n", element.name, element.age, element.user_type);
        ui_cmpts::insert_line();
    }
}

