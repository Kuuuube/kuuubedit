#[derive(Debug)]
pub enum KuuubeditError {
    CommandName,
    CommandParams,
    CommandBuffer,
    CommandUndo,
    InputOutputMatch,
    OutputFileNone
}

impl std::error::Error for KuuubeditError {}

impl std::fmt::Display for KuuubeditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            KuuubeditError::CommandParams => "Incorrect number of command params",
            KuuubeditError::CommandName => "Invalid command name",
            KuuubeditError::CommandBuffer => "This command is only available when the `--no-buf` arg is passed",
            KuuubeditError::CommandUndo => "Undo is only available when the `--undo` arg is passed",
            KuuubeditError::InputOutputMatch => "Cannot overwrite currently open file",
            KuuubeditError::OutputFileNone => "Expected output file Some but found None"
        };
        write!(f, "Kuuubedit error: {message}")
    }
}
