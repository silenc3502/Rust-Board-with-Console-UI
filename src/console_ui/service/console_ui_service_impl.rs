use crate::board::entity::board::Board;
use crate::console_ui::service::console_ui_service::ConsoleUiService;

pub struct ConsoleUiServiceImpl;

impl ConsoleUiServiceImpl {
    pub fn new() -> Self {
        ConsoleUiServiceImpl
    }
}

impl ConsoleUiService for ConsoleUiServiceImpl {
    fn render_board_list(&self, board_list: Vec<Board>) {
        println!("==== 게시판 목록 ====");

        for (index, board) in board_list.iter().enumerate() {
            println!(
                "{}. 제목: {}, 작성자: {}, 등록일: {}",
                index + 1,
                board.get_title(),
                board.get_writer(),
                board.get_create_at()
            );
        }
    }

    fn render_board_register(&self) {
        // 등록 화면 출력 및 필요한 로직
    }

    fn render_board_read(&self) {
        // 조회 화면 출력 및 필요한 로직
    }

    fn render_board_modify(&self) {
        // 수정 화면 출력 및 필요한 로직
    }

    fn render_board_delete(&self) {
        // 삭제 화면 출력 및 필요한 로직
    }
}