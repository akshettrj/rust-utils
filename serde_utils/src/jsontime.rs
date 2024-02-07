use time::OffsetDateTime;

#[derive(Clone, Debug)]
pub struct UnixMilliString(pub OffsetDateTime);

impl<'de> serde::Deserialize<'de> for UnixMilliString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let unix_milli: i128 = s.parse().map_err(serde::de::Error::custom)?;
        let unix_nano = unix_milli * 1_000_000;
        Ok(Self(
            OffsetDateTime::from_unix_timestamp_nanos(unix_nano)
                .map_err(serde::de::Error::custom)?,
        ))
    }
}

impl serde::Serialize for UnixMilliString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let unix_milli = (self.0.unix_timestamp_nanos() / 1_000_000) as i64;
        serializer.serialize_str(&unix_milli.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct UnixMicroString(pub OffsetDateTime);

impl<'de> serde::Deserialize<'de> for UnixMicroString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let unix_micro: i128 = s.parse().map_err(serde::de::Error::custom)?;
        Ok(Self(
            OffsetDateTime::from_unix_timestamp_nanos(unix_micro * 1000)
                .map_err(serde::de::Error::custom)?,
        ))
    }
}

impl serde::Serialize for UnixMicroString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let unix_micro: i128 = self.0.unix_timestamp_nanos() / 1000;
        serializer.serialize_str(&unix_micro.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct UnixNanoString(pub OffsetDateTime);

impl<'de> serde::Deserialize<'de> for UnixNanoString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let unix_nano: i128 = s.parse().map_err(serde::de::Error::custom)?;
        Ok(Self(
            OffsetDateTime::from_unix_timestamp_nanos(unix_nano)
                .map_err(serde::de::Error::custom)?,
        ))
    }
}

impl serde::Serialize for UnixNanoString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.unix_timestamp_nanos().to_string())
    }
}

#[derive(Clone, Debug)]
pub struct UnixString(pub OffsetDateTime);

impl<'de> serde::Deserialize<'de> for UnixString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let unix_sec: i64 = s.parse().map_err(serde::de::Error::custom)?;
        Ok(Self(
            OffsetDateTime::from_unix_timestamp(unix_sec).map_err(serde::de::Error::custom)?,
        ))
    }
}

impl serde::Serialize for UnixString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.unix_timestamp().to_string())
    }
}
