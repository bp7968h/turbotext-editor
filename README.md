# turbotext-editor
TurboText Editor is a lightweight, terminal-based text editor inspired by Vim and born from the curosity of `How does a text editor work!`. It aims to provide a fast and efficient text editing experience with a minimalistic interface and powerful features.

## Features
[x] `Alternate Screen`: Similar to vim, TurboText Editor uses alternate screen and after exisiting, returns to main terminal.
[x] `Modal Editing`: Similar to Vim, TurboText Editor uses different modes for inserting and navigating text.
[ ] `Syntax Highlighting`: Supports syntax highlighting for various programming languages.
[ ] `Customizable Keybindings`: Allows users to customize keybindings to suit their workflow.
[ ] `Plugin Support`: Extend functionality with plugins.

## Installation
To install TurboText Editor, clone the repository and build it using Cargo:
```bash
git clone https://github.com/bp7968h/turbotext-editor.git
cd turbotext-editor
cargo build --release
```
## Usage
Run the editor from the terminal:
```bash
./target/release/turbotext-editor <filename>
```

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
