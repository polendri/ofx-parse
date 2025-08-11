use serde::Deserialize;
use std::collections::HashMap;
use time::OffsetDateTime;

use crate::de::{deserialize_datetime, deserialize_option_datetime};

pub use self::header::*;

pub mod header;

/// Status Severity `<SEVERITY>`, OFX Spec v1.6 3.1.5
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Info,
    Warn,
    Error,
}

/// Status Aggregate `<STATUS>`, OFX Spec v1.6 3.1.5
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename = "STATUS", rename_all = "UPPERCASE")]
pub struct StatusV1<'a> {
    pub code: i32,
    pub severity: Severity,
    pub message: &'a str,
}

/// Signon Response `<SONRS>`, OFX Spec v1.6 2.5.1.2
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename = "SONRS", rename_all = "UPPERCASE")]
pub struct SignonResponse<'a> {
    #[serde(borrow)]
    pub status: StatusV1<'a>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub dtserver: OffsetDateTime,
    #[serde(default)]
    pub userkey: Option<&'a str>,
    #[serde(default, deserialize_with = "deserialize_option_datetime")]
    pub tskeyexpire: Option<OffsetDateTime>,
    pub language: &'a str,
    #[serde(flatten)]
    pub unknown: HashMap<&'a str, &'a str>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename = "SIGNONMSGSRSV1", rename_all = "UPPERCASE")]
pub struct SignonMessageSetV1<'a> {
    // sonrq
    #[serde(borrow)]
    pub sonrs: Option<SignonResponse<'a>>,
}

/// An OFX response document.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename = "OFX", rename_all = "UPPERCASE")]
pub struct OfxRoot<'a> {
    // TODO: support v2
    #[serde(borrow)]
    pub signonmsgsrsv1: Option<SignonMessageSetV1<'a>>,
}

/// An OFX document.
#[derive(Clone, Debug, PartialEq)]
pub struct Ofx<'a> {
    /// The header section of the document.
    pub header: OfxHeader<'a>,
    /// The root of the document.
    pub ofx: OfxRoot<'a>,
}
