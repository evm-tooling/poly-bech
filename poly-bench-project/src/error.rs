use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Project operation failed: {context}")]
    Operation {
        context: String,
        #[source]
        source: std::io::Error,
    },

    #[error(
        "Command failed: {command}\n  cwd: {cwd}\n  status: {status}\n  details:\n{stderr_excerpt}\n  hint: {hint}"
    )]
    CommandFailed {
        command: String,
        cwd: String,
        status: String,
        stderr_excerpt: String,
        hint: String,
    },
}

impl ProjectError {
    pub fn command_failed(
        command: impl Into<String>,
        cwd: impl Into<String>,
        status: impl Into<String>,
        stderr_excerpt: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self::CommandFailed {
            command: command.into(),
            cwd: cwd.into(),
            status: status.into(),
            stderr_excerpt: stderr_excerpt.into(),
            hint: hint.into(),
        }
    }
}
