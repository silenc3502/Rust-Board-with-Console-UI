use std::collections::HashMap;
use uuid::Uuid;
use crate::board::entity::board::Board;
use crate::board::repository::board_repository::BoardRepository;

pub struct InMemoryBoardRepository {
    board_db: HashMap<Uuid, Board>,
}

impl InMemoryBoardRepository {
    pub fn new() -> InMemoryBoardRepository {
        InMemoryBoardRepository {
            board_db: HashMap::new(),
        }
    }
}

impl BoardRepository for InMemoryBoardRepository {
    fn find_by_id(&self, board_id: Uuid) -> Option<Board> {
        self.board_db.get(&board_id).cloned()
    }

    fn save(&mut self, board: Board) -> Board {
        self.board_db.insert(*board.get_board_id(), board.clone());
        board
    }

    fn find_all(&self) -> Vec<Board> {
        self.board_db.values().cloned().collect()
    }

    fn delete_by_id(&mut self, board_id: Uuid) {
        self.board_db.remove(&board_id);
    }
}

#[test]
fn test_save_and_find_by_id() {
    let mut repository = InMemoryBoardRepository::new();
    let board = Board::new(String::from("Test Title"),
                           String::from("Test Writer"),
                           String::from("Test Content"));

    let saved_board = repository.save(board.clone());

    assert_eq!(saved_board, board);

    let board_id = board.get_board_id();
    let retrieved_board = repository.find_by_id(*board_id).unwrap();

    assert_eq!(retrieved_board, board);
}

#[test]
fn test_find_all() {
    let mut repository = InMemoryBoardRepository::new();
    let board1 = Board::new(String::from("Title 1"),
                            String::from("Writer 1"),
                            String::from("Content 1"));
    let board2 = Board::new(String::from("Title 2"),
                            String::from("Writer 2"),
                            String::from("Content 2"));

    repository.save(board1.clone());
    repository.save(board2.clone());

    let all_boards = repository.find_all();

    assert_eq!(all_boards.len(), 2);
    assert!(all_boards.contains(&board1));
    assert!(all_boards.contains(&board2));
}

#[test]
fn test_find_by_id() {
    let mut repository = InMemoryBoardRepository::new();
    let board = Board::new(String::from("Test Title"),
                           String::from("Test Writer"),
                           String::from("Test Content"));

    repository.save(board.clone());
    println!("Repository state after save: {:?}", repository.find_all());

    let board_id = board.get_board_id();

    println!("Searching for board with ID: {:?}", board_id);
    let retrieved_board = repository.find_by_id(*board_id).unwrap_or_else(|| {
        panic!("Failed to retrieve board with id: {:?}", board_id);
    });

    println!("Retrieved board: {:?}", retrieved_board);

    assert_eq!(retrieved_board, board);

    println!("Repository state after find_by_id: {:?}", repository.find_all());
}


#[test]
fn test_delete_by_id() {
    let mut repository = InMemoryBoardRepository::new();
    let board_id = Uuid::new_v4();
    let board = Board::new(
        String::from("Test Title"),
        String::from("Test Writer"),
        String::from("Test Content"),
    );

    repository.save(board.clone());

    println!("Before Deletion: {:?}", repository.find_all());

    repository.delete_by_id(board_id);

    println!("After Deletion: {:?}", repository.find_all());

    assert!(repository.find_by_id(board_id).is_none());
}
