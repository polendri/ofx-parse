use core::{fmt, marker::PhantomData};
use serde::{de, Deserializer};
use time::OffsetDateTime;

use crate::{de::sgml::from_str as from_sgml_str, ofx::Ofx, parse::sgml::datetime};

pub(crate) mod sgml;

pub(super) struct OffsetDateTimeVisitor<T: ?Sized>(pub(super) PhantomData<T>);

impl<'a> serde::de::Visitor<'a> for OffsetDateTimeVisitor<OffsetDateTime> {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an `OffsetDateTime`")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<OffsetDateTime, E> {
        datetime::<nom::error::Error<&str>>(value)
            .map(|(_, dt)| dt)
            .map_err(E::custom)
    }
}

pub(super) struct OptionOffsetDateTimeVisitor<T: ?Sized>(pub(super) PhantomData<T>);

impl<'a> serde::de::Visitor<'a> for OptionOffsetDateTimeVisitor<Option<OffsetDateTime>> {
    type Value = Option<OffsetDateTime>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an `Option<OffsetDateTime>`")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserialize_datetime(deserializer).map(Some)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

/// Deserializer for OFX datetimes into `time::OffsetDateTime`
pub(crate) fn deserialize_datetime<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<OffsetDateTime, D::Error> {
    deserializer.deserialize_str(OffsetDateTimeVisitor(PhantomData))
}

/// Deserializer for optional OFX datetimes into `time::OffsetDateTime`
pub(crate) fn deserialize_option_datetime<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<OffsetDateTime>, D::Error> {
    deserializer.deserialize_option(OptionOffsetDateTimeVisitor(PhantomData))
}

/// Deserializes an OFX document from a string.
pub fn from_str(s: &str) -> crate::error::Result<Ofx> {
    from_sgml_str(s)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn deserialize_datetime__valid_input__
}
