use crate::board::entity::board::Board;
use uuid::Uuid;

pub trait BoardRepository {
    fn find_all(&self) -> Vec<Board>;
    fn save(&mut self, board: Board) -> Board;
    fn find_by_id(&self, board_id: Uuid) -> Option<Board>;
    fn delete_by_id(&mut self, board_id: Uuid);
}