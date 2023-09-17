use crate::error::ProxyError;

pub(crate) enum RequestRole {
    Human,
    Bot,
}

impl TryFrom<&str> for RequestRole {
    type Error = ProxyError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "human" => Ok(Self::Human),
            "bot" => Ok(Self::Bot),
            _ => Err(ProxyError::UnknownAction),
        }
    }
}
