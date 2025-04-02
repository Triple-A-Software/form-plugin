use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Postgres, prelude::Type};
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JavascriptDate {
    inner: DateTime<Utc>,
}

impl TS for JavascriptDate {
    type WithoutGenerics = Self;
    fn decl() -> String {
        panic!("JavascriptDate cannot be declared");
    }
    fn decl_concrete() -> String {
        panic!("JavascriptDate cannot be declared");
    }
    fn name() -> String {
        "string".to_string()
    }
    fn inline() -> String {
        "string".to_string()
    }
    fn inline_flattened() -> String {
        panic!("JavascriptDate cannot be flattened");
    }
}

impl Type<Postgres> for JavascriptDate {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        NaiveDateTime::type_info()
    }
}

impl<'r> Decode<'r, Postgres> for JavascriptDate {
    fn decode(
        value: <Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let inner = <NaiveDateTime as Decode<Postgres>>::decode(value)?;
        let tz_offset = FixedOffset::east_opt(0).unwrap();
        let fixed = tz_offset.from_utc_datetime(&inner);
        let inner = Utc.from_utc_datetime(&fixed.naive_utc());
        Ok(JavascriptDate { inner })
    }
}

impl<'q> Encode<'q, Postgres> for JavascriptDate {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let naive = self.inner.naive_utc();
        naive.encode_by_ref(buf)
    }
}

impl Serialize for JavascriptDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for JavascriptDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = DateTime::deserialize(deserializer)?;
        Ok(JavascriptDate { inner })
    }
}
