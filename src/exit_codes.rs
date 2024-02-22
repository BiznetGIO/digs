#[derive(Debug, PartialEq)]
pub enum ExitCode {
    Success,
    GeneralError,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        match code {
            ExitCode::Success => 0,
            ExitCode::GeneralError => 1,
        }
    }
}
