use serde::Deserialize;

#[macro_export]
macro_rules! serde_deserialize_from_str {
    ($type:ident) => {
        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(
                deserializer: D,
            ) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                std::str::FromStr::from_str(&s).map_err(serde::de::Error::custom)
            }
        }
    };
}

#[macro_export]
macro_rules! serde_serialize_to_string {
    ($type:ident) => {
        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let s = self.to_string();
                serializer.serialize_str(&s)
            }
        }
    };
}

pub fn deserialize_from_string_parse<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr + serde::Deserialize<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber<T> {
        String(String),
        Number(T),
    }

    match StringOrNumber::<T>::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrNumber::Number(i) => Ok(i),
    }
}

pub fn serialize_to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: ToString,
{
    let s = value.to_string();
    serializer.serialize_str(&s)
}
