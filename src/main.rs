pub mod helpers;
use repl::helpers::{helper_fn, structs_enums, colors, ui_cmpts};
use repl::*;

fn main() {
    ui_cmpts::insert_line();
    let user_in = helper_fn::display_then_read(
        "W E L C O M E - T O - lilREPL\n\
To start, enter: [start / s]\n",
    );
    ui_cmpts::insert_line();

    let mut program_status: bool = false;

    if user_in == String::from("start") || user_in == String::from("s") {
        program_status = true;
    }
    let mut profile_storage: Vec<structs_enums::Profile> = Vec::new();
    let mut feeling_board_storage: Vec<structs_enums::FeelingBoard> = Vec::new();
    //let mut sensitive_storage:HashMap<,> = HashMap::new();

    while program_status {
        println!();
        ui_cmpts::insert_line();
        println!("{}Hello!{}", colors::GREEN, colors::RESET);
        ui_cmpts::insert_line();

        let greeting_choice: NavChoice = navigate(helper_fn::string_to_int(
            helper_fn::display_then_read(
                "Navigation:\n1. Build profile:\n2. View profile\n3. Find Profile\n4. Feeling Board\n5. Keyword detect\n6. File Explorer\n\n",
            ),
        ));

        let end_or_no: bool = match greeting_choice {
            NavChoice::ProfileBuilder => build_profile(&mut profile_storage),
            NavChoice::ProfileViewer => view_stored_profiles(&profile_storage),
            NavChoice::ProfileFinder => profile_finder(&profile_storage),
            NavChoice::FeelingBoard => feeling_board_build(&mut feeling_board_storage),
            NavChoice::KeywordFunctions => keyword_detect(),
            NavChoice::FileExplorer => ui_cmpts::not_done_notice(),
            NavChoice::NotSelected => quit_or_cont(),
        };

        program_status = end_or_no
    }
}
