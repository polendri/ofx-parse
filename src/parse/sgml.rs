//! Parsers for SGML-based (v1.x) OFX documents.

use nom::{
    branch::alt,
    bytes::complete::{take, take_until},
    character::complete::{char, i8, multispace0, u16, u8},
    combinator::{map_opt, map_parser, opt, success, verify},
    error::ParseError,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

pub(crate) mod element;
pub(crate) mod header;

/// Consumes whitespace before the provided parser.
pub(crate) fn whitespace_preceded<'a, P>(
    mut p: P,
) -> impl FnMut(&'a str) -> IResult<&'a str, P::Output, P::Error>
where
    P: Parser<&'a str>,
{
    move |input: &str| {
        let (input, _) = multispace0(input)?;
        p.parse(input)
    }
}

/// Parses a datetime.
pub(crate) fn datetime<'a, E>(input: &'a str) -> IResult<&'a str, OffsetDateTime, E>
where
    E: ParseError<&'a str>,
{
    let (input, (year, month, day, (hour, minute, second, millisecond, offset))) =
        whitespace_preceded((
            map_parser(take(4usize), u16),
            map_parser(
                take(2usize),
                map_opt(u8, |m: u8| match m {
                    1 => Some(Month::January),
                    2 => Some(Month::February),
                    3 => Some(Month::March),
                    4 => Some(Month::April),
                    5 => Some(Month::May),
                    6 => Some(Month::June),
                    7 => Some(Month::July),
                    8 => Some(Month::August),
                    9 => Some(Month::September),
                    10 => Some(Month::October),
                    11 => Some(Month::November),
                    12 => Some(Month::December),
                    _ => None,
                }),
            ),
            map_parser(take(2usize), verify(u8, |d| *d >= 1 && *d <= 31)),
            alt((
                (
                    map_parser(take(2usize), verify(u8, |h| *h <= 60)),
                    map_parser(take(2usize), verify(u8, |m| *m <= 60)),
                    map_parser(take(2usize), verify(u8, |s| *s <= 60)),
                    alt((
                        preceded(char('.'), map_parser(take(3usize), u16)),
                        success(0),
                    )),
                    alt((
                        delimited(
                            whitespace_preceded(char('[')),
                            whitespace_preceded(verify(i8, |o| *o >= -12 && *o <= 14)),
                            whitespace_preceded((opt((char(':'), take_until("]"))), char(']'))),
                        ),
                        success(0),
                    )),
                ),
                success((0, 0, 0, 0, 0)),
            )),
        ))
        .parse(input)?;

    Ok((
        input,
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(year.into(), month, day).unwrap(),
            Time::from_hms_milli(hour, minute, second, millisecond).unwrap(),
            UtcOffset::from_hms(offset, 0, 0).unwrap(),
        ),
    ))
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;
    use test_case::test_case;
    use time::{macros::datetime, OffsetDateTime};

    use crate::parse::test_utils::{assert_parser, Expected};

    const MIDNIGHT_UTC: OffsetDateTime = datetime!(2025-01-01 0:00 UTC);
    const ARBITRARY_UTC: OffsetDateTime = datetime!(2025-01-02 3:04:05.006 UTC);
    const ARBITRARY_EST: OffsetDateTime = datetime!(2025-01-02 3:04:05.006 -5);
    const ARBITRARY_CET: OffsetDateTime = datetime!(2025-01-02 3:04:05.006 +2);

    #[test_case(""                      , Err(ErrorKind::Eof), ""     ; "eof"                    )]
    #[test_case("2025"                  , Err(ErrorKind::Eof), ""     ; "yy"                     )]
    #[test_case("202501"                , Err(ErrorKind::Eof), ""     ; "yymm"                   )]
    #[test_case("20250101"              , Ok(MIDNIGHT_UTC)   , ""     ; "yymmdd"                 )]
    #[test_case("2025010100"            , Ok(MIDNIGHT_UTC)   , "00"   ; "yymmddhh"               )]
    #[test_case("202501010000"          , Ok(MIDNIGHT_UTC)   , "0000" ; "yymmddhhmm"             )]
    #[test_case("20250101000000"        , Ok(MIDNIGHT_UTC)   , ""     ; "yymmddhhmmss"           )]
    #[test_case("20250101000000."       , Ok(MIDNIGHT_UTC)   , "."    ; "yymmddhhmmss."          )]
    #[test_case("20250102030405.006"    , Ok(ARBITRARY_UTC)  , ""     ; "yymmddhhmmss.mmm"       )]
    #[test_case("20250102030405[-5]"    , Ok(datetime!(2025-01-02 3:04:05 -5)), "" ; "offset, no milliseconds")]
    #[test_case("20250102030405.006[-5]", Ok(ARBITRARY_EST)  , ""     ; "offset, no name"        )]
    #[test_case("20250102030405.006[-5:EST]", Ok(ARBITRARY_EST), ""   ; "offset, with name"      )]
    #[test_case("20250102030405.006 [ -5 ]", Ok(ARBITRARY_EST), ""    ; "offset, with whitespace")]
    #[test_case("20250102030405.006 [ -5 : EST ]", Ok(ARBITRARY_EST), "" ; "offset, with name & whitespace")]
    #[test_case("20250102030405.006[+2:CET]", Ok(ARBITRARY_CET), "" ; "offset, with +")]
    fn datetime(input: &str, expected: Expected<OffsetDateTime>, remaining: &str) {
        assert_parser(super::datetime, input, expected, remaining);
    }
}
