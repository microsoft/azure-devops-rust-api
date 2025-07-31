// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Azure DevOps date-time serde support.
//!
//! Protocol date-time fields are usually RFC3339 format.
//! However, there is one special case value where
//! services sometimes send `0001-01-01T00:00:00` which
//! is not RFC3339 compliant (no offset), so we need to
//! have a custom deserializer to handle this gracefully.

use azure_core::error::{ErrorKind, ResultExt};
use serde::de;
use std::fmt;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// Returns the given date-time as a String in RFC3339 format
pub fn format_date_time(date_time: &OffsetDateTime) -> azure_core::error::Result<String> {
    date_time
        .format(&Rfc3339)
        .with_context(ErrorKind::DataConversion, || {
            format!("Failed to format date_time: {date_time}")
        })
}

pub mod rfc3339 {
    use super::*;

    pub use time::serde::rfc3339::serialize;

    #[allow(dead_code)]
    pub fn deserialize<'de, D>(d: D) -> Result<OffsetDateTime, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(DateTimeVisitor)
    }

    struct DateTimeVisitor;

    impl de::Visitor<'_> for DateTimeVisitor {
        type Value = OffsetDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "RFC3339 datetime string or 0001-01-01T00:00:00")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let value = match value {
                "0001-01-01T00:00:00" => "0001-01-01T00:00:00Z",
                _ => value,
            };

            // Some services return a time without an offset, which is not RFC3339 compliant.
            // If the parse fails, try adding a Z to the end and try again.
            match OffsetDateTime::parse(value, &Rfc3339) {
                Ok(dt) => Ok(dt),
                Err(e) => {
                    if let Ok(dt) = OffsetDateTime::parse(&format!("{value}Z"), &Rfc3339) {
                        Ok(dt)
                    } else {
                        Err(E::custom(format!("Parse error {e} for {value}")))
                    }
                }
            }
        }
    }

    pub mod option {
        use super::*;
        pub use time::serde::rfc3339::option::serialize;

        #[allow(dead_code)]
        pub fn deserialize<'de, D>(d: D) -> Result<Option<OffsetDateTime>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            d.deserialize_option(OptionalDateTimeVisitor)
        }

        struct OptionalDateTimeVisitor;

        impl<'de> de::Visitor<'de> for OptionalDateTimeVisitor {
            type Value = Option<OffsetDateTime>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "null or a datetime string")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_some<D>(self, d: D) -> Result<Option<OffsetDateTime>, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                Ok(Some(d.deserialize_str(DateTimeVisitor)?))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::{ErrorKind, ResultExt};
    use serde::{Deserialize, Serialize};
    use serde_json;

    pub fn parse_rfc3339(s: &str) -> azure_core::Result<OffsetDateTime> {
        OffsetDateTime::parse(s, &Rfc3339).with_context(ErrorKind::DataConversion, || {
            format!("unable to parse rfc3339 date '{s}")
        })
    }

    #[derive(Serialize, Deserialize)]
    struct ExampleState {
        #[serde(with = "crate::date_time::rfc3339")]
        created_time: time::OffsetDateTime,

        #[serde(default, with = "crate::date_time::rfc3339::option")]
        deleted_time: Option<time::OffsetDateTime>,
    }

    #[test]
    fn test_serde_datetime() {
        let json_state = r#"{
            "created_time": "2021-07-01T10:45:02Z"
        }"#;
        let state: ExampleState = serde_json::from_str(json_state).unwrap();
        assert_eq!(
            parse_rfc3339("2021-07-01T10:45:02Z").unwrap(),
            state.created_time
        );
        assert_eq!(state.deleted_time, None);
    }

    #[test]
    fn test_serde_datetime_beginning_of_time_without_offset() {
        let json_state = r#"{
            "created_time": "0001-01-01T00:00:00"
        }"#;
        let state: ExampleState = serde_json::from_str(json_state).unwrap();
        assert_eq!(
            parse_rfc3339("0001-01-01T00:00:00Z").unwrap(),
            state.created_time
        );
        assert_eq!(state.deleted_time, None);
    }

    #[test]
    fn test_serde_datetime_beginning_of_time_with_offset() {
        let json_state = r#"{
            "created_time": "0001-01-01T00:00:00Z"
        }"#;
        let state: ExampleState = serde_json::from_str(json_state).unwrap();
        assert_eq!(
            state.created_time,
            parse_rfc3339("0001-01-01T00:00:00Z").unwrap()
        );
        assert_eq!(state.deleted_time, None);
    }

    #[test]
    fn test_serde_datetime_invalid_time() {
        let json_state = r#"{
            "created_time": "0002-01-01T00:0000"
        }"#;
        let result: Result<ExampleState, _> = serde_json::from_str(json_state);
        assert!(result.is_err());
    }

    #[test]
    fn test_serde_datetime_without_offset() {
        let json_state = r#"{
            "created_time": "2023-05-03T20:09:17.5460824"
        }"#;
        let state: ExampleState = serde_json::from_str(json_state).unwrap();
        assert_eq!(
            state.created_time,
            parse_rfc3339("2023-05-03T20:09:17.5460824Z").unwrap()
        );
    }

    #[test]
    fn test_serde_datetime_optional_time() {
        let json_state = r#"{
            "created_time": "2022-03-04T00:01:02Z",
            "deleted_time": "2022-03-04T01:02:03Z"
        }"#;
        let state: ExampleState = serde_json::from_str(json_state).unwrap();
        assert_eq!(
            state.created_time,
            parse_rfc3339("2022-03-04T00:01:02Z").unwrap()
        );
        assert_eq!(
            state.deleted_time,
            Some(parse_rfc3339("2022-03-04T01:02:03Z").unwrap())
        );
    }

    #[test]
    fn test_serde_datetime_optional_beginning_of_time() {
        let json_state = r#"{
            "created_time": "2022-03-04T00:01:02Z",
            "deleted_time": "0001-01-01T00:00:00"
        }"#;
        let state: ExampleState = serde_json::from_str(json_state).unwrap();
        assert_eq!(
            state.created_time,
            parse_rfc3339("2022-03-04T00:01:02Z").unwrap()
        );
        assert_eq!(
            state.deleted_time,
            Some(parse_rfc3339("0001-01-01T00:00:00Z").unwrap())
        );
    }
}
