use crate::board::entity::board::Board;

pub trait ConsoleUiService {
    fn render_board_list(&self, board_list: Vec<Board>);
    fn render_board_register(&self);
    fn render_board_read(&self);
    fn render_board_modify(&self);
    fn render_board_delete(&self);
}