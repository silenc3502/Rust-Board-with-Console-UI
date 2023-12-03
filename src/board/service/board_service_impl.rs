use uuid::Uuid;
use crate::board::entity::board::Board;
use crate::board::repository::board_repository::BoardRepository;
use crate::board::service::board_service::BoardService;
use crate::board::service::request::board_request::BoardRequest;

pub struct BoardServiceImpl {
    board_repository: Box<dyn BoardRepository>,
}

impl BoardServiceImpl {
    pub fn new(board_repository: Box<dyn BoardRepository>) -> BoardServiceImpl {
        BoardServiceImpl {
            board_repository,
        }
    }
}

impl BoardService for BoardServiceImpl {
    fn register(&mut self, board_request: BoardRequest) -> Board {
        let board = board_request.to_board();
        self.board_repository.save(board)
    }

    fn list(&self) -> Vec<Board> {
        self.board_repository.find_all()
    }

    fn read(&self, board_id: Uuid) -> Option<Board> {
        self.board_repository.find_by_id(board_id)
    }

    fn remove(&mut self, board_id: Uuid) {
        self.board_repository.delete_by_id(board_id);
    }

    fn modify(&mut self, board_id: Uuid, board_request: BoardRequest) -> Option<Board> {
        let maybe_board = self.board_repository.find_by_id(board_id);

        match maybe_board {
            Some(mut board) => {
                board.set_title(board_request.title().to_string());
                board.set_content(board_request.content().to_string());

                self.board_repository.save(board.clone());

                Some(board)
            },
            None => {
                println!("Board 정보를 찾지 못했습니다: {:?}", board_id);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::repository::in_memory_board_repository::InMemoryBoardRepository;

    #[test]
    fn test_register() {
        let repository = InMemoryBoardRepository::new();
        let mut service = BoardServiceImpl::new(Box::new(repository));
        let board_request = BoardRequest::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string());

        service.register(board_request);

        let boards = service.list();
        assert_eq!(boards.len(), 1);
        assert_eq!(boards[0].get_title(), "Test Title");
        // Add more assertions as needed.
    }

    #[test]
    fn test_list() {
        let repository = InMemoryBoardRepository::new();
        let service = BoardServiceImpl::new(Box::new(repository));
        // Ensure the service returns an empty list initially.
        let boards = service.list();
        assert_eq!(boards.len(), 0);
    }

    #[test]
    fn test_read() {
        let repository = InMemoryBoardRepository::new();
        let mut service = BoardServiceImpl::new(Box::new(repository));
        let board_request = BoardRequest::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string());

        let registered_board = service.register(board_request);
        let board_id = registered_board.get_board_id().clone();

        let retrieved_board = service.read(board_id.clone());
        assert_eq!(retrieved_board.unwrap().get_title(), "Test Title");
    }

    #[test]
    fn test_remove() {
        let repository = InMemoryBoardRepository::new();
        let mut service = BoardServiceImpl::new(Box::new(repository));
        let board_request = BoardRequest::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string());
        let board_id = service.register(board_request).get_board_id().clone();

        service.remove(board_id.clone());

        let boards = service.list();
        assert_eq!(boards.len(), 0);
    }

    #[test]
    fn test_modify() {
        let repository = InMemoryBoardRepository::new();
        let mut service = BoardServiceImpl::new(Box::new(repository));
        let board_request = BoardRequest::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string());
        let board_id = service.register(board_request).get_board_id().clone();
        let updated_board_request = BoardRequest::new(
            "Updated Title".to_string(),
            "Updated Writer".to_string(),
            "Updated Content".to_string());

        let modified_board = service.modify(board_id.clone(), updated_board_request);
        assert_eq!(modified_board.unwrap().get_title(), "Updated Title");

        let retrieved_board = service.read(board_id.clone());
        assert_eq!(retrieved_board.unwrap().get_title(), "Updated Title");
    }
}
