use crate::board::entity::board::Board;

#[derive(Debug)]
pub struct BoardRequest {
    title: String,
    writer: String,
    content: String,
}

// BoardRequest에 대한 getter 메서드 구현
impl BoardRequest {
    pub fn new(title: String, writer: String, content: String) -> BoardRequest {
        BoardRequest { title, writer, content }
    }
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn writer(&self) -> &str {
        &self.writer
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn to_board(&self) -> Board {
        Board::new(self.title.clone(), self.writer.clone(), self.content.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let title = String::from("Sample Title");
        let writer = String::from("John Doe");
        let content = String::from("Sample Content");

        let board_request = BoardRequest::new(title.clone(), writer.clone(), content.clone());
        let board = board_request.to_board();

        assert_eq!(board.get_title(), &title);
        assert_eq!(board.get_writer(), &writer);
        assert_eq!(board.get_content(), &content);
    }
}
