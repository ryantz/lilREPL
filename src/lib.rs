pub mod helpers;
use helpers::*;
use structs_enums::Show;

use std::time::SystemTime;

#[derive(PartialEq)]
pub enum NavChoice {
    ProfileBuilder,
    ProfileViewer,
    ProfileFinder,
    FeelingBoard,
    KeywordFunctions,
    FileExplorer,
    NotSelected,
}

pub fn navigate(choice: u8) -> NavChoice {

    let chosen_route: NavChoice = match choice {
        1 => NavChoice::ProfileBuilder,
        2 => NavChoice::ProfileViewer,
        3 => NavChoice::ProfileFinder,
        4 => NavChoice::FeelingBoard,
        5 => NavChoice::KeywordFunctions,
        6 => NavChoice::FileExplorer,
        _ => NavChoice::NotSelected,
    };

    chosen_route
}

pub fn build_profile(ref_profile_storage: &mut Vec<structs_enums::Profile>) -> bool {
    ui_cmpts::profile_builder_greeting();

    let name = helper_fn::display_then_read("Please enter your name: ");
    let age = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your age: "));
    let selection = helper_fn::string_to_int(helper_fn::display_then_read(
        "Please enter your user type (1: Admin 2: User, 3: No selection): ",
    ));

    let user_profile = structs_enums::Profile::build(name, age, selection);
    ui_cmpts::insert_line();

    println!("{}",user_profile.show());

    ui_cmpts::insert_line();

    save_profile(user_profile, ref_profile_storage);

    true
}

pub fn save_profile(
    profile_to_save: structs_enums::Profile,
    ref_profile_storage: &mut Vec<structs_enums::Profile>,
) -> &mut Vec<structs_enums::Profile> {
    ref_profile_storage.push(profile_to_save);

    ref_profile_storage
}

pub fn view_stored_profiles(ref_profile_storage: &Vec<structs_enums::Profile>) -> bool {
    // profile_storage borrows ownership of Vec

    ui_cmpts::profile_viewer_greeting();

    for i in 0..ref_profile_storage.len() {
        ui_cmpts::insert_line();
        println!(
            "\nstorage id: {:?}\nname: {}\nage: {}\nuser type: {:?}\n",
            i + 1,
            ref_profile_storage[i].name,
            ref_profile_storage[i].age,
            ref_profile_storage[i].user_type
        );
        ui_cmpts::insert_line();
    }
    true
}

pub fn profile_finder(ref_profile_storage: &Vec<structs_enums::Profile>) -> bool {
    ui_cmpts::profile_finder_greeting();

    let user_id = helper_fn::string_to_usize(helper_fn::display_then_read("Please enter the id: "));
    find_profile_by_id(user_id, ref_profile_storage);

    true
}

pub fn find_profile_by_id(
    id: usize,
    ref_profile_storage: &Vec<structs_enums::Profile>,
) -> &structs_enums::Profile {
    let index = id - 1;
    // slice
    let ref_single_val = &ref_profile_storage[index];

    ref_single_val
}

pub fn quit_or_cont() -> bool {
    let user_input: String = helper_fn::display_then_read(
        "you selected an option outside the range! Do you want to quit the program instead? [yes / y / end]\n",
    );

    if user_input == String::from("yes")
        || user_input == String::from("y")
        || user_input == String::from("end")
    {
        false
    } else {
        true
    }
}

pub fn keyword_detect() -> bool {
    let user_input = helper_fn::display_then_read("\n");
    // find first word
    let key: &str = helper_fn::read_first_word(&user_input);

    match key {
        keywords::SHOUT => println!(
            "shouting:{} \n",
            helper_fn::disp_after_keyword(&user_input).to_uppercase()
        ),
        _ => println!("No keyword detected"),
    }
    true
}

pub fn save_feeling_board(
    feeling_board: structs_enums::FeelingBoard,
    ref_feeling_board_storage: &mut Vec<structs_enums::FeelingBoard>,
) -> &mut Vec<structs_enums::FeelingBoard> {
    ref_feeling_board_storage.push(feeling_board);

    ref_feeling_board_storage
}

// WARN: System time is not what i thought
// TODO: Use Chrono crate
pub fn feeling_board_build(ref_feeling_board_storage: &mut Vec<structs_enums::FeelingBoard>) -> bool {
    let user_feeling_selection: u8 = helper_fn::string_to_int(helper_fn::display_then_read(
        "What are you feeling right now?\n1.Happy\n2.Sad\n3.Hopeful\n3.Despair\n4.Excited\n5.Motivated\n6.Nothing Much..\n",
    ));

    // actually just time
    let date = SystemTime::now();
    let feeling_board_entry = structs_enums::FeelingBoard::build(user_feeling_selection, date);

    let show_preview = feeling_board_entry.show();

    println!("{}", show_preview);

    save_feeling_board(feeling_board_entry, ref_feeling_board_storage);
    true
}
