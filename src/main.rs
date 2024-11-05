use std::io;
use turbotext_editor::Editor;

fn main() -> anyhow::Result<()>{
    let mut stdout = io::stdout();
    
    let mut turbotext_editor = Editor::new();

    turbotext_editor.init(&mut stdout)
}
