mod helpers;
use helpers::*;

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
    let mut profile_storage: Vec<Profile> = Vec::new();

    loop{

        ui_cmpts::profile_builder_greeting();

        let greeting_choice: NavChoice = navigate(helper_fn::string_to_int(helper_fn::display_then_read("Navigation:\n1. Build profile,\n2. View profile\n\n")));

        if greeting_choice == NavChoice::BUILDER {
            let name = helper_fn::display_then_read("Please enter your name: ");
            let age = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your age: "));
            let selection = helper_fn::string_to_int(helper_fn::display_then_read("Please enter your user type: "));

            let user_profile = Profile::build(name, age, selection);

            ui_cmpts::insert_line();

            //user_profile.show();
            save_profile(user_profile, &mut profile_storage);
            view_stored_profiles(&profile_storage);
        } else {
            ui_cmpts::not_done_notice();
        }
        ui_cmpts::insert_line();
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


// TODO:: need to use a vec. 
fn save_profile(profile_to_save: Profile,  profile_storage:&mut Vec<Profile>) -> &mut Vec<Profile>{
    profile_storage.push(profile_to_save);
    
    profile_storage
}

fn view_stored_profiles(profile_storage: &Vec<Profile>) {
    for element in profile_storage {
        println!("{:?}", element);
    }


}
