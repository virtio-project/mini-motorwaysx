use actix_web::error::ParseError;
use actix_web::http::header::{Header, IntoHeaderValue, InvalidHeaderValue};
use actix_web::http::{HeaderName, HeaderValue};
use actix_web::HttpMessage;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub steam_id: u64,
    pub name: String,
    pub score: u64,
    pub status: GameStatus,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameStatus {
    Initialized,
    Started,
    GameOver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statics {
    pub steam_name: String,
    pub steam_id: u64,
    pub build_id: u64,
    pub game_language: String,
    pub app_owner: u64,
}

mod header_names {
    pub static STEAM_ID: &str = "X-Steam-Id";
    pub static STEAM_NAME: &str = "X-Steam-Name";
    pub static SCREENSHOT_NAME: &str = "X-Screenshot-Name";
}
use header_names::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SteamId(u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamName(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotName(String);

impl Display for SteamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoHeaderValue for SteamId {
    type Error = Infallible;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        Ok(HeaderValue::from(self.0))
    }
}

impl Header for SteamId {
    fn name() -> HeaderName {
        HeaderName::from_static(STEAM_ID)
    }

    fn parse<T: HttpMessage>(msg: &T) -> Result<Self, ParseError> {
        let value = msg
            .headers()
            .get(STEAM_ID)
            .ok_or(ParseError::Header)?
            .to_str()
            .map_err(|_| ParseError::Header)?
            .parse::<u64>()
            .map_err(|_| ParseError::Header)?;
        Ok(Self(value))
    }
}

impl Display for SteamName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoHeaderValue for SteamName {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(self.0.as_str())
    }
}

impl Header for SteamName {
    fn name() -> HeaderName {
        HeaderName::from_static(STEAM_NAME)
    }

    fn parse<T: HttpMessage>(msg: &T) -> Result<Self, ParseError> {
        let value = msg
            .headers()
            .get(STEAM_NAME)
            .ok_or(ParseError::Header)?
            .to_str()
            .map_err(|_| ParseError::Header)?;
        Ok(Self(value.to_owned()))
    }
}

impl Display for ScreenshotName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoHeaderValue for ScreenshotName {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(self.0.as_str())
    }
}

impl Header for ScreenshotName {
    fn name() -> HeaderName {
        HeaderName::from_static(SCREENSHOT_NAME)
    }

    fn parse<T: HttpMessage>(msg: &T) -> Result<Self, ParseError> {
        let value = msg
            .headers()
            .get(SCREENSHOT_NAME)
            .ok_or(ParseError::Header)?
            .to_str()
            .map_err(|_| ParseError::Header)?;
        Ok(Self(value.to_owned()))
    }
}
