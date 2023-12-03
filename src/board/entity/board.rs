use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    board_id: Uuid,
    title: String,
    writer: String,
    content: String,
    create_at: DateTime<Utc>,
    update_at: DateTime<Utc>,
}

impl Board {
    pub fn new(title: String, writer: String, content: String) -> Board {
        let now = Utc::now();
        Board {
            board_id: Uuid::new_v4(),
            title,
            writer,
            content,
            create_at: now,
            update_at: now,
        }
    }

    pub fn get_board_id(&self) -> &Uuid {
        &self.board_id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_writer(&self) -> &str {
        &self.writer
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_create_at(&self) -> &DateTime<Utc> {
        &self.create_at
    }

    pub fn get_update_at(&self) -> &DateTime<Utc> {
        &self.update_at
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_update_at(&mut self) {
        self.update_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_create_board_with_content() {
        let title = String::from("Sample Title");
        let writer = String::from("John Doe");
        let content = String::from("Sample Content");

        let board = Board::new(title.clone(), writer.clone(), content.clone());

        assert_eq!(board.get_title(), &title);
        assert_eq!(board.get_writer(), &writer);
        assert_eq!(board.get_content(), &content);
    }
}
