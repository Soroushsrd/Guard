use editor::Editor;

pub mod editor;
pub mod terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
