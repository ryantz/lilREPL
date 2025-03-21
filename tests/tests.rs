//some tests
use repl::find_profile_by_id;
use repl::helpers::structs_enums;

#[test]
fn test_fn_find_people_by_id() {
    let prof1 = structs_enums::Profile {
        name: String::from("ry"),
        age: 28,
        user_type: structs_enums::UserType::Admin,
    };

    let prof2 = structs_enums::Profile {
        name: String::from("bob"),
        age: 22,
        user_type: structs_enums::UserType::User,
    };

    let prof3 = structs_enums::Profile {
        name: String::from("black"),
        age: 20,
        user_type: structs_enums::UserType::NotSelected,
    };

    let ans1 = structs_enums::Profile {
        name: String::from("ry"),
        age: 28,
        user_type: structs_enums::UserType::Admin,
    };

    let ans2 = structs_enums::Profile {
        name: String::from("bob"),
        age: 22,
        user_type: structs_enums::UserType::User,
    };

    let ans3 = structs_enums::Profile {
        name: String::from("black"),
        age: 20,
        user_type: structs_enums::UserType::NotSelected,
    };
    let test_storage: Vec<structs_enums::Profile> = vec![prof1, prof2, prof3];

    assert_eq!(find_profile_by_id(1, &test_storage), &ans1);
    assert_eq!(find_profile_by_id(2, &test_storage), &ans2);
    assert_eq!(find_profile_by_id(3, &test_storage), &ans3);
}

