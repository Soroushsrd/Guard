use editor::Editor;
pub mod documents;
pub mod editor;
pub mod files;
pub mod highlights;
pub mod lines;
pub mod terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
