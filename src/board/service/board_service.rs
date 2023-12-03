use uuid::Uuid;
use crate::board::entity::board::Board;
use crate::board::service::request::board_request::BoardRequest;


pub trait BoardService {
    fn register(&mut self, board_request: BoardRequest) -> Board;
    fn list(&self) -> Vec<Board>;
    fn read(&self, board_id: Uuid) -> Option<Board>;
    fn remove(&mut self, board_id: Uuid);
    fn modify(&mut self, board_id: Uuid, board_request: BoardRequest) -> Option<Board>;
}