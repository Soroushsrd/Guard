mod terminal;
mod editor;

fn main() {
    let mut editor = editor::Editor::new();
    editor.run();
}