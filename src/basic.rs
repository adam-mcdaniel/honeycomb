use crate::{
    atoms::{opt, seq_no_ws, space, sym},
    language::{alpha, alphanumeric, identifier, numeral},
    transform::collect,
    Parser,
};

use core::fmt::{Display, Error, Formatter};

use alloc::string::{String, ToString};

/// Parses an email address and returns the component preceding the '@' symbol
/// and the domain following the '@' symbol as a tuple.
pub fn email() -> Parser<(String, String)> {
    // Must start with alphabetic character
    ((alpha().is()
        >> (
            // Every other character in the first component must be alphanumeric,
            // or one of: '-', '_', or '.'
            ((alphanumeric() | sym('-') | sym('_') | sym('.')) * (..)) - collect
            // The domain is composed of two alphanumeric identifiers
        )
        & sym('@') >> (identifier() & (sym('.') >> identifier())))
        - |email: (String, (String, String))| {
            let address = email.0;
            let domain = (email.1).0 + "." + &(email.1).1;

            (address, domain)
        }) % "a valid email address"
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhoneNumber {
    pub country_code: Option<String>,
    pub area_code: String,
    pub prefix: String,
    pub line_number: String,
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if let Some(country_code) = self.country_code.clone() {
            write!(f, "+{}-", country_code)?;
        }
        write!(f, "{}-{}-{}", self.area_code, self.prefix, self.line_number)
    }
}

/// Consumes a phone number
pub fn phone_number() -> Parser<PhoneNumber> {
    let country_code = seq_no_ws("+") >> ((numeral() * (..3)) - collect);
    let area_code = opt(seq_no_ws("-")) >> ((numeral() * (3..3)) - collect);
    let prefix = opt(seq_no_ws("-")) >> ((numeral() * (3..3)) - collect);
    let line_number = opt(seq_no_ws("-")) >> ((numeral() * (4..4)) - collect);

    ((opt(country_code) & (area_code & (prefix & line_number)))
        - |s: (Option<String>, (String, (String, String)))| PhoneNumber {
            country_code: s.0,
            area_code: (s.1).0,
            prefix: ((s.1).1).0,
            line_number: ((s.1).1).1,
        }) % "a valid phone number"
        | ((((space() >> numeral()) * (13..13)) << space())
            - collect
            - |s: String| PhoneNumber {
                country_code: Some(s[0..3].to_string()),
                area_code: s[3..6].to_string(),
                prefix: s[6..9].to_string(),
                line_number: s[9..13].to_string(),
            }) % "a valid phone number"
        | ((((space() >> numeral()) * (10..10)) << space())
            - collect
            - |s: String| PhoneNumber {
                country_code: None,
                area_code: s[0..3].to_string(),
                prefix: s[3..6].to_string(),
                line_number: s[6..10].to_string(),
            }) % "a valid phone number"
}
