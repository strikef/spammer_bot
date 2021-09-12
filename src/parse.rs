use std::collections::VecDeque;
use std::fmt::Result as fmtResult;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum CommandErrorKind {
    InvalidCommand,
    NotProvided,
}

#[derive(Debug)]
pub struct CommandError {
    kind: CommandErrorKind,
    user_value: String,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        match self.kind {
            CommandErrorKind::InvalidCommand => {
                write!(f, "{} is not a valid command", self.user_value)
            }
            CommandErrorKind::NotProvided => write!(f, "command must be provided!"),
        }
    }
}

impl CommandError {
    fn new(user_value: String) -> Self {
        let kind = if user_value.trim().is_empty() {
            CommandErrorKind::NotProvided
        } else {
            CommandErrorKind::InvalidCommand
        };
        Self { kind, user_value }
    }
}

#[derive(Debug)]
enum UsernameErrorKind {
    InvalidUsername,
    NotProvided,
}

#[derive(Debug)]
pub struct UsernameError {
    kind: UsernameErrorKind,
    user_value: String,
}

impl Display for UsernameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "{} does not seem to exist or is not a valid username",
            self.user_value
        )
    }
}

impl UsernameError {
    fn new(user_value: String) -> Self {
        let kind = if user_value.trim().is_empty() {
            UsernameErrorKind::NotProvided
        } else {
            UsernameErrorKind::InvalidUsername
        };
        Self { kind, user_value }
    }
}

#[derive(Debug)]
pub struct IntervalError {
    user_value: String,
}

impl Display for IntervalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "{} can't be converted into numeric interval value",
            self.user_value
        )
    }
}

impl IntervalError {
    fn new(user_value: String) -> Self {
        Self { user_value }
    }
}

#[derive(Debug)]
pub struct RepetitionError {
    user_value: String,
}

impl Display for RepetitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "{} can't be converted into integer repetition count value",
            self.user_value
        )
    }
}

impl RepetitionError {
    fn new(user_value: String) -> Self {
        Self { user_value }
    }
}

#[derive(Debug)]
pub struct CountError {
    user_value: String,
}

impl Display for CountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "{} can't be converted into integer mention count value",
            self.user_value
        )
    }
}

impl CountError {
    fn new(user_value: String) -> Self {
        Self { user_value }
    }
}

#[derive(Debug)]
pub enum ParseError {
    CommandError(CommandError),
    UsernameError(UsernameError),
    IntervalError(IntervalError),
    RepetitionError(RepetitionError),
    CountError(CountError),
}

impl From<CommandError> for ParseError {
    fn from(error: CommandError) -> Self {
        Self::CommandError(error)
    }
}

impl From<UsernameError> for ParseError {
    fn from(error: UsernameError) -> Self {
        Self::UsernameError(error)
    }
}

impl From<IntervalError> for ParseError {
    fn from(error: IntervalError) -> Self {
        Self::IntervalError(error)
    }
}

impl From<RepetitionError> for ParseError {
    fn from(error: RepetitionError) -> Self {
        Self::RepetitionError(error)
    }
}

impl From<CountError> for ParseError {
    fn from(error: CountError) -> Self {
        Self::CountError(error)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        match self {
            Self::CommandError(e) => write!(f, "couldn't parse command: {}", e),
            Self::UsernameError(e) => write!(f, "couldn't parse username: {}", e),
            Self::IntervalError(e) => write!(f, "couldn't parse interval: {}", e),
            Self::RepetitionError(e) => write!(f, "couldn't parse repetition count: {}", e),
            Self::CountError(e) => write!(f, "couldn't parse mention count: {}", e),
        }
    }
}

#[derive(Debug)]
pub struct SpamForArgs {
    pub interval: f32,
    pub repeat: usize,
}

impl SpamForArgs {
    fn parse_args(mut args: VecDeque<String>) -> Result<Self, ParseError> {
        let interval = args
            .pop_front()
            .map(|arg| arg.parse().map_err(|_| IntervalError::new(arg)))
            .unwrap_or(Ok(1.0))?;
        let repeat = args
            .pop_front()
            .map(|arg| arg.parse().map_err(|_| RepetitionError::new(arg)))
            .unwrap_or(Ok(5))?;
        Ok(Self { interval, repeat })
    }
}

#[derive(Debug)]
pub struct SpamOnceArgs {
    pub repeat: usize,
}

impl SpamOnceArgs {
    fn parse_args(mut args: VecDeque<String>) -> Result<Self, ParseError> {
        let repeat = args
            .pop_front()
            .map(|arg| arg.parse().map_err(|_| CountError::new(arg)))
            .unwrap_or(Ok(5))?;
        Ok(Self { repeat })
    }
}

#[derive(Debug)]
pub enum SpamArgsKind {
    SpamFor(SpamForArgs),
    SpamOnce(SpamOnceArgs),
}

impl From<SpamForArgs> for SpamArgsKind {
    fn from(args: SpamForArgs) -> Self {
        Self::SpamFor(args)
    }
}

impl From<SpamOnceArgs> for SpamArgsKind {
    fn from(args: SpamOnceArgs) -> Self {
        Self::SpamOnce(args)
    }
}

#[derive(Debug)]
pub struct SpamArgs {
    pub kind: SpamArgsKind,
    pub username: String,
}

impl SpamArgs {
    fn parse_spam_for(mut args: VecDeque<String>) -> Result<Self, ParseError> {
        let username = Self::parse_username(&mut args)?;
        let kind = SpamForArgs::parse_args(args)?.into();
        Ok(Self { kind, username })
    }

    fn parse_spam_once(mut args: VecDeque<String>) -> Result<Self, ParseError> {
        let username = Self::parse_username(&mut args)?;
        let kind = SpamOnceArgs::parse_args(args)?.into();
        Ok(Self { kind, username })
    }

    fn parse_username(args: &mut VecDeque<String>) -> Result<String, ParseError> {
        let username = args.pop_front().unwrap_or_default();
        if username.is_empty() {
            Err(UsernameError::new(username))?
        } else {
            Ok(username)
        }
    }
}

pub fn parse_command(command: String) -> Result<SpamArgs, ParseError> {
    // use VecDeque, as args will be parsed (popped) from front
    let mut args: VecDeque<String> = command
        .split_whitespace()
        .map(|word| word.to_string())
        .collect();

    let command = args.pop_front().unwrap_or_default();
    if command == "'spo" {
        SpamArgs::parse_spam_once(args)
    } else if command == "'sp" {
        SpamArgs::parse_spam_for(args)
    } else {
        Err(CommandError::new(command))?
    }
}
