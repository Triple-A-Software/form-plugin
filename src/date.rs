use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
#[cfg(feature = "seed")]
use fake::Dummy;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Postgres, prelude::Type};
use ts_rs::TS;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavascriptDate {
    inner: DateTime<Utc>,
}

#[cfg(feature = "seed")]
impl<T> Dummy<T> for JavascriptDate {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_config: &T, rng: &mut R) -> Self {
        use fake::{Fake, Faker};

        Self {
            inner: Faker.fake_with_rng(rng),
        }
    }
}

impl TS for JavascriptDate {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;
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

impl From<DateTime<Utc>> for JavascriptDate {
    fn from(value: DateTime<Utc>) -> Self {
        Self { inner: value }
    }
}

impl From<JavascriptDate> for DateTime<Utc> {
    fn from(value: JavascriptDate) -> Self {
        value.inner
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
