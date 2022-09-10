use std::rc::Rc;
use crate::application::handlers::createuser_handler::CreateUserHandler;
use crate::infrastructure::domain::repositories::inmemoryuser_repository::InMemoryUserRepository;
use crate::application::handlers::getallusers_handler::GetAllUsersHandler;
use crate::application::handlers::getuser_handler::GetUserHandler;
use crate::application::handlers::updateuser_handler::UpdateUserHandler;
use crate::application::requests::createuser_request::CreateUserRequest;
use crate::application::requests::getuser_request::GetUserRequest;
use crate::application::requests::updateuser_request::UpdateUserRequest;
use crate::domain::entities::gender_type::Gender;
use crate::presentation::prompt::{ask_question, menu};

mod application;
mod domain;
mod infrastructure;
mod presentation;

fn main() {

    let user_repository = Rc::new(InMemoryUserRepository::new_with_samples());

    let get_all_users_handler = GetAllUsersHandler::new(user_repository.clone());
    let get_user_handler = GetUserHandler::new(user_repository.clone());
    let create_user_handler = CreateUserHandler::new(user_repository.clone());
    let update_user_handler = UpdateUserHandler::new(user_repository.clone());

    while {
        let option: u8 = menu(
            std::io::stdin().lock(),
            std::io::stdout(),
            std::io::stderr(),
            vec![
                "List all user",
                "Read a user",
                "Create a user",
                "Update a user",
            ],
        );
        match option {
            1 => {
                let users = get_all_users_handler.execute();

                print!("{}", users);
            }
            2 => {
                let user_id = ask_question(
                    std::io::stdin().lock(),
                    std::io::stdout(),
                    "Please, enter the ID of the user that you want to read:",
                );

                println!();

                let get_user_req = GetUserRequest::new(user_id.parse().unwrap());

                let user = get_user_handler.execute(get_user_req);

                match user {
                    Ok(c) => println!("{}", c),
                    Err(e) => eprintln!("ERROR: {}", e),
                }
            }
            3 => {
                let user_name = ask_question(
                    std::io::stdin().lock(),
                    std::io::stdout(),
                    "Please, enter the name of the client that you want to create:",
                );
                let user_age: i32 = ask_question(
                    std::io::stdin().lock(),
                    std::io::stdout(),
                    "\nEnter the age of the user that you want to create:",
                ).parse().unwrap();

                let user_gender_string = ask_question(
                    std::io::stdin().lock(),
                    std::io::stdout(),
                    "\nEnter the gender (female/male) of the user that you want to create:",
                );
                let user_gender = parse_gender(user_gender_string);

                let create_user_req = CreateUserRequest::new(user_name, user_age,user_gender);

                create_user_handler.execute(create_user_req);

                println!("\nUser created!");
            }
            4 => {
                let user_id: i32 = ask_question(
                    std::io::stdin().lock(),
                    std::io::stdout(),
                    "Please, enter the ID of the user that you want to read:",
                ).parse().unwrap();

                let user_name = ask_question(
                    std::io::stdin().lock(),
                    std::io::stdout(),
                    "\nEnter the new name of the user:",
                );
                let user_age: i32 = ask_question(
                    std::io::stdin().lock(),
                    std::io::stdout(),
                    "\nEnter the new age of the user:",
                ).parse().unwrap();

                let user_gender_string = ask_question(
                    std::io::stdin().lock(),
                    std::io::stdout(),
                    "\nEnter the new gender (female/male) of the user that you want to update:",
                );
                let user_gender = parse_gender(user_gender_string);

                let result = update_user_handler.execute(UpdateUserRequest::new(
                    user_id,user_name,user_age,user_gender));

                match result {
                    Ok(_) => println!("\nUser edited!"),
                    Err(e) => eprintln!("\nERROR: {}", e),
                }
            }
            0 => println!("Exiting..."),
            _ => println!("Invalid option"),
        };
        println!();

        option != 0
    }{}

}

//terrible solution
fn parse_gender(user_gender_string: String) -> Gender{
    let mut user_gender: Gender = Gender::Male;
    match user_gender_string.as_str() {
        "male" => {
            user_gender = Gender::Male;
        }
        "female" => {
            user_gender = Gender::Female;
        }
        _ => {
            println!("Wrong gender");
        }
    }
    user_gender
}
