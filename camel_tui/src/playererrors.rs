
#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
    InvalidConfiguration,
}

#[derive(Debug)]
pub enum PlaceError {
    InvalidIndex,
    InvalidColor,
}

#[derive(Debug)]
pub enum PlayerActionError {
    MoveError(MoveError),
    PlaceError(PlaceError),
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::InvalidMove => write!(f, "invalid move"),
            MoveError::InvalidConfiguration => write!(f, "invalid configuration"),
        }
    }
}

impl std::fmt::Display for PlaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceError::InvalidIndex => write!(f, "invalid index"),
            PlaceError::InvalidColor => write!(f, "invalid color"),
        }
    }
}

impl std::fmt::Display for PlayerActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerActionError::MoveError(e) => write!(f, "move error: {}", e),
            PlayerActionError::PlaceError(e) => write!(f, "place error: {}", e),
        }
    }
}

impl std::error::Error for MoveError {}
impl std::error::Error for PlaceError {}
impl std::error::Error for PlayerActionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PlayerActionError::MoveError(e) => Some(e),
            PlayerActionError::PlaceError(e) => Some(e),
        }
    }
}

impl From<MoveError> for PlayerActionError {
    fn from(err: MoveError) -> Self {
        PlayerActionError::MoveError(err)
    }
}

impl From<PlaceError> for PlayerActionError {
    fn from(err: PlaceError) -> Self {
        PlayerActionError::PlaceError(err)
    }
}
