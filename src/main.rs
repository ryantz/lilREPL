mod helpers;
use helpers::*;

#[derive(PartialEq)]
enum NavChoice {
    ProfileBuilder,
    ProfileViewer,
    NotSelected,
}

fn main() {
    let mut profile_storage: Vec<structs_enums::Profile> = Vec::new();

    loop {
        println!("");
        ui_cmpts::insert_line();
        println!("{}Hello! PS: To exit the program, please use <C-c>{}", colors::GREEN, colors::RESET);
        ui_cmpts::insert_line();

        let greeting_choice: NavChoice = navigate(helper_fn::string_to_int(helper_fn::display_then_read("Navigation:\n1. Build profile:\n2. View profile\n3. File Navigation\n\n")));

        if greeting_choice == NavChoice::ProfileBuilder {
            profile_builder(&mut profile_storage);

        } else if greeting_choice == NavChoice::ProfileViewer {

            view_stored_profiles(&profile_storage);           

        } else {
            ui_cmpts::not_done_notice();
        }
    }
}

fn navigate(choice: u8) -> NavChoice {
    let chosen_route = match choice {
        1 => NavChoice::ProfileBuilder,
        2 => NavChoice::ProfileViewer,
        _ => NavChoice::NotSelected,
    };

    chosen_route
}


fn save_profile(profile_to_save: structs_enums::Profile,  profile_storage:&mut Vec<structs_enums::Profile>) -> &mut Vec<structs_enums::Profile>{
    profile_storage.push(profile_to_save);
    
    profile_storage
}

fn view_stored_profiles(profile_storage: &Vec<structs_enums::Profile>) {
    ui_cmpts::profile_viewer_greeting();

    for element in profile_storage {
        ui_cmpts::insert_line();
        println!("\nname: {}\nage: {}\nuser type: {:?}\n", element.name, element.age, element.user_type);
        ui_cmpts::insert_line();
    }
}

fn profile_builder(profile_storage: &mut Vec<structs_enums::Profile>) {
    ui_cmpts::profile_builder_greeting();

    let name = helper_fn::display_then_read("Please enter your name: ");
    let age = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your age: "));
    let selection = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your user type (1: Admin 2: User, 3: No selection): "));

    let user_profile = structs_enums::Profile::build(name, age, selection);
    ui_cmpts::insert_line();

    user_profile.show();

    ui_cmpts::insert_line();

    save_profile(user_profile, profile_storage);

}
