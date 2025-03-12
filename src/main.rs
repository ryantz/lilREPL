mod helpers;
use helpers::*;

#[derive(PartialEq)]
enum NavChoice {
    ProfileBuilder,
    ProfileViewer,
    //ProfileFinder,
    FileExplorer,
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

        //if greeting_choice == NavChoice::ProfileBuilder {
        //    profile_builder(&mut profile_storage);

        //} else if greeting_choice == NavChoice::ProfileViewer {

        //    view_stored_profiles(&profile_storage);           

        //} else if greeting_choice == NavChoice::FileExplorer{
        //    todo!();
        //} else {
        //    ui_cmpts::not_done_notice();
        //}
        match greeting_choice {
            NavChoice::ProfileBuilder => build_profile(&mut profile_storage),
            NavChoice::ProfileViewer => view_stored_profiles(&profile_storage),
            //NavChoice::ProfileFinder => find_profile_by_id(id),
            NavChoice::FileExplorer => ui_cmpts::not_done_notice(),
            NavChoice::NotSelected => ui_cmpts::not_done_notice(),
        }
    }
}

fn navigate(choice: u8) -> NavChoice {
    let chosen_route = match choice {
        1 => NavChoice::ProfileBuilder,
        2 => NavChoice::ProfileViewer,
        //3 => NavChoice::ProfileFinder,
        3 => NavChoice::FileExplorer,
        _ => NavChoice::NotSelected,
    };

    chosen_route
}

fn build_profile(ref_profile_storage: &mut Vec<structs_enums::Profile>) {
    ui_cmpts::profile_builder_greeting();

    let name = helper_fn::display_then_read("Please enter your name: ");
    let age = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your age: "));
    let selection = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your user type (1: Admin 2: User, 3: No selection): "));

    let user_profile = structs_enums::Profile::build(name, age, selection);
    ui_cmpts::insert_line();

    user_profile.show();

    ui_cmpts::insert_line();

    save_profile(user_profile, ref_profile_storage);
}

fn save_profile(profile_to_save: structs_enums::Profile,  ref_profile_storage:&mut Vec<structs_enums::Profile>) -> &mut Vec<structs_enums::Profile>{
    ref_profile_storage.push(profile_to_save);

    ref_profile_storage
}

fn view_stored_profiles(ref_profile_storage: &Vec<structs_enums::Profile>) { 
    // profile_storage borrows ownership of Vec
    
    ui_cmpts::profile_viewer_greeting();

    for i in 0..ref_profile_storage.len() {
        ui_cmpts::insert_line();
        println!("\nstorage id: {:?}\nname: {}\nage: {}\nuser type: {:?}\n", i+1 , ref_profile_storage[i].name, ref_profile_storage[i].age, ref_profile_storage[i].user_type);
        ui_cmpts::insert_line();
    }
}

// hello from desktop
fn find_profile_by_id(id: u8) {
    todo!();
}
