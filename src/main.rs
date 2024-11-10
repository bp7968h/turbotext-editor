use std::io;
use turbotext_editor::{Buffer, Editor};

fn main() -> anyhow::Result<()> {
    let arg = std::env::args().nth(1);
    let buffer = Buffer::new(arg);
    let mut stdout = io::stdout();

    let mut turbotext_editor = Editor::new(buffer);

    turbotext_editor.init(&mut stdout)
}
