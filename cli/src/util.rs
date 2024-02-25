use std::io;

pub trait GetOrExit<T> {
    fn get_or_exit(self) -> T;
}

impl<T> GetOrExit<T> for Result<Result<T, String>, io::Error> {
    fn get_or_exit(self) -> T {
        match self {
            Ok(Ok(value)) => value,
            Ok(Err(error)) => {
                clap::Error::raw(clap::error::ErrorKind::Format, error.with_new_line()).exit()
            }
            Err(error) => clap::Error::raw(clap::error::ErrorKind::Io, error).exit(),
        }
    }
}

pub trait WithNewLine {
    fn with_new_line(self) -> Self;
}

impl WithNewLine for String {
    fn with_new_line(self) -> Self {
        if self.ends_with('\n') {
            self
        } else {
            format!("{self}\n")
        }
    }
}
