pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    SwitchMode,
    Write(char),
    NewLine,
}