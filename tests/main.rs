use std::collections::HashMap;

use test_case::test_case;
use time::macros::datetime;

use ofx_parse::{error::Result, from_str, ofx::header::*, ofx::*};

fn header() -> OfxHeader {
    OfxHeader {
        header_version: 100,
        data: OfxContentType::OfxSgml,
        version: 102,
        security: OfxSecurity::Type1,
        encoding: OfxEncoding::UsAscii,
        charset: OfxCharset::WindowsLatin1,
        compression: "NONE".to_owned(),
        old_file_uid: "NONE".to_owned(),
        new_file_uid: "NONE".to_owned(),
    }
}

#[test_case(
    include_str!("data/v102/empty.ofx"),
    Ok(Ofx {
        header: header(),
        ofx: OfxRoot::default(),
    }) ;
    "empty OFX element"
)]
#[test_case(
    include_str!("data/v102/signon_response__required_fields.ofx"),
    Ok(Ofx {
        header: header(),
        ofx: OfxRoot {
            signonmsgsrsv1: Some(SignonMessageSetResponseV1 {
                sonrs: Some(SignonResponse {
                    status: StatusV1 {
                        code: 0,
                        severity: Severity::Info,
                        message: "OK".to_owned(),
                    },
                    dtserver: datetime!(2022-07-17 16:41:44 -8),
                    userkey: None,
                    tskeyexpire: None,
                    language: "ENG".to_owned(),
                    unknown: HashMap::from([("INTU.BID".to_owned(), "00015".to_owned())]),
                })
            }),
        },
    }) ;
    "signon response with required fields"
)]
#[test_case(
    include_str!("data/v102/signon_response__all_fields.ofx"),
    Ok(Ofx {
        header: header(),
        ofx: OfxRoot {
            signonmsgsrsv1: Some(SignonMessageSetResponseV1 {
                sonrs: Some(SignonResponse {
                    status: StatusV1 {
                        code: 0,
                        severity: Severity::Info,
                        message: "OK".to_owned(),
                    },
                    dtserver: datetime!(2022-07-17 16:41:44 -8),
                    userkey: Some("ABCDEFG".to_owned()),
                    tskeyexpire: Some(datetime!(2022-08-01 01:02:03 -8)),
                    language: "ENG".to_owned(),
                    unknown: HashMap::from([("INTU.BID".to_owned(), "00015".to_owned())]),
                })
            }),
        },
    }) ;
    "signon response with minimum fields"
)]
fn test_from_str(input: &str, expected: Result<Ofx>) {
    assert_eq!(from_str(input), expected);
}
