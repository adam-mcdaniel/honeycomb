extern crate honeycomb;
use honeycomb::{
    basic::{email, phone_number, PhoneNumber},
    Error,
};

#[test]
fn email_test() {
    assert_eq!(
        email().parse("autumn-dancer@domain.com"),
        Ok((String::from("autumn-dancer"), String::from("domain.com")))
    );

    assert_eq!(
        email().parse("shannon_ballet@domain.org"),
        Ok((String::from("shannon_ballet"), String::from("domain.org")))
    );

    assert_eq!(
        email().parse("snowball.sweet@gmail.com"),
        Ok((String::from("snowball.sweet"), String::from("gmail.com")))
    );

    assert_eq!(
        email().parse("snowball. sweet@gmail.com"),
        Error::new(" ", "@", " sweet@gmail.com")
    );

    assert_eq!(
        email().parse("–autumn-dancer@domain.com"),
        Error::new(
            "–autumn-dancer@domain.com",
            "Not –autumn-dancer@domain.com",
            "–autumn-dancer@domain.com"
        )
    );

    assert_eq!(
        email().parse(".snowball.sweet@domain.com"),
        Error::new(
            ".snowball.sweet@domain.com",
            "Not .snowball.sweet@domain.com",
            ".snowball.sweet@domain.com"
        )
    );
}

#[test]
fn phone_number_test() {
    let test_number = PhoneNumber {
        country_code: Some(1.to_string()),
        area_code: 123.to_string(),
        prefix: 456.to_string(),
        line_number: 7890.to_string(),
    };

    assert_eq!(
        phone_number().parse("+1-123-456-7890"),
        Ok(test_number.clone())
    );

    assert_eq!(
        phone_number().parse("+1 - 123 - 456- 7890"),
        Ok(test_number.clone())
    );

    let test_number = PhoneNumber {
        country_code: None,
        area_code: 123.to_string(),
        prefix: 456.to_string(),
        line_number: 7890.to_string(),
    };

    assert_eq!(
        phone_number().parse("123  - 456     -  7890"),
        Ok(test_number.clone())
    );

    assert_eq!(phone_number().parse("1234567890"), Ok(test_number.clone()));

    assert_eq!(phone_number().parse("123 4567890"), Ok(test_number.clone()));
}
