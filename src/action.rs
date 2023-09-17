use crate::error::ProxyError;

pub(crate) enum Action {
    Randomize,
    Remove,
}

impl TryFrom<&str> for Action {
    type Error = ProxyError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "randomize" => Ok(Self::Randomize),
            "remove" => Ok(Self::Remove),
            _ => Err(ProxyError::UnknownAction),
        }
    }
}
