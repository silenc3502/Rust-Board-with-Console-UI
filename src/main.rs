mod board;
mod console_ui;

use uuid::Uuid;
use chrono::Utc;

use board::entity::board::Board;
use board::repository::in_memory_board_repository::InMemoryBoardRepository;
use crate::board::repository::board_repository::BoardRepository;
use crate::board::service::board_service::BoardService;
use crate::board::service::board_service_impl::BoardServiceImpl;
use crate::board::service::request::board_request::BoardRequest;
use crate::console_ui::controller::console_ui_controller::ConsoleUiController;
use crate::console_ui::service::console_ui_service_impl::ConsoleUiServiceImpl;

pub static mut BOARD_SERVICE: Option<Box<dyn BoardService>> = None;
pub static mut CONSOLE_UI_CONTROLLER: Option<ConsoleUiController> = None;

fn init_config() {
    unsafe {
        let board_repository = InMemoryBoardRepository::new();
        let board_service_impl = BoardServiceImpl::new(
            Box::new(
                board_repository));
        BOARD_SERVICE = Some(Box::new(
            board_service_impl) as Box<dyn BoardService>);

        let console_ui_service = Box::new(
            ConsoleUiServiceImpl::new());

        let board_service: Box<dyn BoardService> = Box::new(
            BoardServiceImpl::new(
                Box::new(
                    InMemoryBoardRepository::new())));

        CONSOLE_UI_CONTROLLER = Some(ConsoleUiController::new(console_ui_service, board_service));
    }
}

fn main() {
    init_config();

    // unsafe {
    //     if let Some(mut service) = BOARD_SERVICE.take() {
    //         let board_request = BoardRequest::new(
    //             "Initial Title".to_string(),
    //             "Initial Writer".to_string(),
    //             "Initial Content".to_string(),
    //         );
    //
    //         let board_response = service.register(board_request);
    //         let board_id = board_response.get_board_id().clone();
    //         println!("Registered board with ID: {:?}", board_id);
    //
    //         let retrieved_board = service.read(board_id.clone());
    //         println!("Retrieved board: {:?}", retrieved_board);
    //
    //         let modified_board_request = BoardRequest::new(
    //             "Modified Title".to_string(),
    //             "Modified Writer".to_string(),
    //             "Modified Content".to_string(),
    //         );
    //         let modified_board_response = service.modify(
    //             board_id.clone(),
    //             modified_board_request);
    //         println!("Modified board: {:?}", modified_board_response);
    //
    //         let second_board_request = BoardRequest::new(
    //             "Second Title".to_string(),
    //             "Second Writer".to_string(),
    //             "Second Content".to_string(),
    //         );
    //         let second_board_response = service.register(
    //             second_board_request);
    //         let second_board_id = second_board_response.get_board_id().clone();
    //         println!("Registered second board with ID: {:?}", second_board_id);
    //
    //         let board_list = service.list();
    //         println!("List of boards: {:?}", board_list);
    //
    //         service.remove(board_id.clone());
    //         println!("Removed board with ID: {:?}", board_id);
    //
    //         let board_list = service.list();
    //         println!("List of boards: {:?}", board_list);
    //     }
    // }

    unsafe {
        let Some(mut ui_controller) = CONSOLE_UI_CONTROLLER.take() else { todo!() };

        ui_controller.route_board_list();
    }
}
