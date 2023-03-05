use std::fmt;

pub enum AccetedEncoding {
    Json,
    Yaml,
    Xml,
    Csv,
}

impl AccetedEncoding {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "application/json" => Some(Self::Json),
            "application/yaml" => Some(Self::Yaml),
            "application/xml" => Some(Self::Xml),
            "text/csv" => Some(Self::Csv),
            _ => None,
        }
    }

    pub fn from_header(headers: axum::http::HeaderMap) -> Option<Self> {
        let ae = headers.get(axum::http::header::ACCEPT_ENCODING)?;
        let ae = ae.to_str().ok()?;
        Self::from_str(ae)
    }
}

impl fmt::Display for AccetedEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json => write!(f, "application/json"),
            Self::Yaml => write!(f, "application/yaml"),
            Self::Xml => write!(f, "application/xml"),
            Self::Csv => write!(f, "text/csv"),
        }
    }
}
