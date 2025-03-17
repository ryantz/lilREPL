mod helpers;
use helpers::*;

#[derive(PartialEq)]
enum NavChoice {
    ProfileBuilder,
    ProfileViewer,
    ProfileFinder,
    KeywordFunctions,
    FileExplorer,
    NotSelected,
}

fn main() {

    ui_cmpts::insert_line();
    let user_in = helper_fn::display_then_read("W E L C O M E - T O - lilREPL\nTo start, enter: [start / s]\n");
    ui_cmpts::insert_line();
    
    let mut program_status: bool = false;

    if user_in == String::from("start") || user_in == String::from("s") {
        program_status = true;
    }
    let mut profile_storage: Vec<structs_enums::Profile> = Vec::new();
    //let mut sensitive_storage:HashMap<,> = HashMap::new();

    while program_status {

        println!();
        ui_cmpts::insert_line();
        println!("{}Hello!{}", colors::GREEN, colors::RESET);
        ui_cmpts::insert_line();

        let greeting_choice: NavChoice = navigate(helper_fn::string_to_int(helper_fn::display_then_read("Navigation:\n1. Build profile:\n2. View profile\n3. Find Profile\n4. Keyword detect\n5. File Explorer\n\n")));
        
        let end_or_no = match greeting_choice {
            NavChoice::ProfileBuilder => build_profile(&mut profile_storage),
            NavChoice::ProfileViewer => view_stored_profiles(&profile_storage),
            NavChoice::ProfileFinder => profile_finder(&profile_storage),
            NavChoice::KeywordFunctions => keyword_detect(), 
            NavChoice::FileExplorer => ui_cmpts::not_done_notice(),
            NavChoice::NotSelected => quit_or_cont(),
        };

        program_status = end_or_no
    }
}

fn navigate(choice: u8) -> NavChoice {
    let chosen_route = match choice {
        1 => NavChoice::ProfileBuilder,
        2 => NavChoice::ProfileViewer,
        3 => NavChoice::ProfileFinder,
        4 => NavChoice::KeywordFunctions,
        5 => NavChoice::FileExplorer,
        _ => NavChoice::NotSelected,
    };

    chosen_route
}

fn build_profile(ref_profile_storage: &mut Vec<structs_enums::Profile>) -> bool {
    ui_cmpts::profile_builder_greeting();

    let name = helper_fn::display_then_read("Please enter your name: ");
    let age = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your age: "));
    let selection = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your user type (1: Admin 2: User, 3: No selection): "));

    let user_profile = structs_enums::Profile::build(name, age, selection);
    ui_cmpts::insert_line();

    user_profile.show();

    ui_cmpts::insert_line();

    save_profile(user_profile, ref_profile_storage);
    
    true
}

fn save_profile(profile_to_save: structs_enums::Profile,  ref_profile_storage:&mut Vec<structs_enums::Profile>) -> &mut Vec<structs_enums::Profile>{
    ref_profile_storage.push(profile_to_save);

    ref_profile_storage
}

fn view_stored_profiles(ref_profile_storage: &Vec<structs_enums::Profile>) -> bool { 
    // profile_storage borrows ownership of Vec
    
    ui_cmpts::profile_viewer_greeting();

    for i in 0..ref_profile_storage.len() {
        ui_cmpts::insert_line();
        println!("\nstorage id: {:?}\nname: {}\nage: {}\nuser type: {:?}\n", i+1 , ref_profile_storage[i].name, ref_profile_storage[i].age, ref_profile_storage[i].user_type);
        ui_cmpts::insert_line();
    }
    true
}

// hello from desktop
fn profile_finder(ref_profile_storage: &Vec<structs_enums::Profile>) -> bool {
    ui_cmpts::profile_finder_greeting();

    let user_id = helper_fn::string_to_usize(helper_fn::display_then_read("Please enter the id: "));
    find_profile_by_id(user_id, ref_profile_storage);
    
    true
}

fn find_profile_by_id(id: usize, ref_profile_storage: &Vec<structs_enums::Profile>) -> &structs_enums::Profile {
    let index = id - 1;
    // slice
    let ref_single_val = &ref_profile_storage[index];

    ref_single_val
}

fn quit_or_cont() -> bool {
    let user_input:String = helper_fn::display_then_read("you selected an option outside the range! Do you want to quit the program instead? [yes / y / end]\n");
    
    if user_input == String::from("yes") || user_input == String::from("y") || user_input == String::from("end") {
        false
    } else {
        true
    }
}

fn keyword_detect() -> bool {
    let user_input = helper_fn::display_then_read("\n");
    // find first word
    let key: &str = helper_fn::read_first_word(&user_input);
     // test
    match key {
        keywords::SHOUT => println!("shouting:{} \n" , helper_fn::disp_after_keyword(&user_input).to_uppercase()),
        _ => println!("No keyword detected"),
    }
    true
}


