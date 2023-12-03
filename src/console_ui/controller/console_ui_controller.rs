use uuid::Uuid;
use crate::board::entity::board::Board;
use crate::console_ui::service::console_ui_service::ConsoleUiService;

use crate::BOARD_SERVICE;
use crate::board::service::board_service::BoardService;
use crate::board::service::request::board_request::BoardRequest;

use std::io::{self, Write};

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");  // Flush stdout before reading input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn clear_input_buffer() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to clear input buffer");
}

pub struct ConsoleUiController {
    console_ui_service: Box<dyn ConsoleUiService>,
    board_service: Box<dyn BoardService>,
}

impl ConsoleUiController {
    pub fn new(
        console_ui_service: Box<dyn ConsoleUiService>,
        board_service: Box<dyn BoardService>,
    ) -> ConsoleUiController {
        ConsoleUiController {
            console_ui_service,
            board_service,
        }
    }

    fn read_post(controller: &mut ConsoleUiController, board_list: Vec<Board>) {
        println!("Enter the post number to read: ");

        let mut post_number = String::new();
        std::io::stdin().read_line(&mut post_number).expect("Failed to read line");
        let command: usize = post_number.trim().parse().unwrap_or(0);

        if command > 0 && command <= board_list.len() {
            let selected_board = &board_list[command - 1];
            controller.route_board_read(selected_board.get_board_id().clone());
        } else {
            println!("Invalid post number");
        }
    }

    pub fn route_board_list(&mut self) {
        let board_list = self.board_service.list();

        self.console_ui_service.render_board_list(board_list.clone());

        let menu_options: [(&str, Box<dyn Fn(&mut ConsoleUiController, Vec<Board>) >); 3] = [
            ("Write a new post", Box::new(|controller, _| {
                Self::route_board_register(controller);
            })),
            ("Read a post", Box::new(move |controller, board_list| {
                Self::read_post(controller, board_list);
            })),
            ("Go back", Box::new(|_, _| { })),
        ];

        println!("Options:");
        for (index, (option, _)) in menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let command: usize = input.trim().parse().unwrap_or(0);

        println!("command: {}", command);
        if command > 0 && command <= menu_options.len() {
            let (_, action) = &menu_options[command - 1];
            action(self, board_list.clone());  // Pass board_list to the closure
        } else {
            println!("Invalid command");
        }
    }

    pub fn route_board_register(&mut self) {
        let title = get_user_input("Enter the title: ");
        let writer = get_user_input("Enter the writer: ");
        let content = get_user_input("Enter the content: ");

        let board_request = BoardRequest::new(title, writer, content);

        let board = self.board_service.register(board_request);
        println!("board: {:?}", board);
        println!("board.get_board_id().clone(): {:?}", board.get_board_id().clone());
        self.route_board_read(board.get_board_id().clone());
    }
    pub fn route_board_read(&mut self, board_id: Uuid) {
        println!("board_id: {:?}", board_id);
        let retrieved_board = self.board_service.read(board_id);
        println!("Retrieved board: {:?}", retrieved_board);

        match retrieved_board {
            Some(board) => {
                println!("==== Post Details ====");
                println!("Title: {}", board.get_title());
                println!("Writer: {}", board.get_writer());
                println!("Content: {}", board.get_content());
                println!("Created At: {}", board.get_create_at());
            }
            None => {
                println!("Failed to retrieve the board details.");
            }
        }

        let menu_options: [(&str, Box<dyn Fn(&mut ConsoleUiController, Uuid)>); 3] = [
            ("Modify post", Box::new(|controller, board_id| {
                controller.route_board_modify(board_id);
            })),
            ("Delete post", Box::new(|controller, board_id| {
                controller.route_board_delete(board_id);
            })),
            ("Go back", Box::new(|controller, _| {
                controller.route_board_list();
            })),
        ];

        println!("Options:");
        for (index, (option, _)) in menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command: usize = input.trim().parse().unwrap_or(0);

        println!("command: {}", command);
        if command > 0 && command <= menu_options.len() {
            let (_, action) = &menu_options[command - 1];
            action(self, board_id);  // Pass board_id to the closure
        } else {
            println!("Invalid command");
        }
    }
    pub fn route_board_modify(&mut self, board_id: Uuid) {
        println!("Enter new title: ");
        let new_title = get_user_input("> ");

        println!("Enter new content: ");
        let new_content = get_user_input("> ");

        let existing_board = match self.board_service.read(board_id) {
            Some(board) => board,
            None => {
                println!("Board not found. Modification aborted.");
                return;
            }
        };

        let board_modify_request = BoardRequest::new(
            new_title,
            existing_board.get_writer().to_string(),
            new_content
        );

        self.board_service.modify(board_id, board_modify_request);
        println!("Board modified successfully!");

        self.route_board_read(board_id);
    }
    pub fn route_board_delete(&mut self, board_id: Uuid) {
        println!("Are you sure you want to delete this post? (yes/no)");
        let confirmation = get_user_input("> ");

        if confirmation.to_lowercase() == "yes" {
            self.board_service.remove(board_id);
            println!("Board deleted successfully!");

            self.route_board_list();
        } else {
            println!("Deletion canceled.");
        }
    }

}