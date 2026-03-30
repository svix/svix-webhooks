use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_bytes::{ByteBuf, Bytes};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Option::<ByteBuf>::deserialize(deserializer)?;
    Ok(buf.map(ByteBuf::into_vec))
}

pub(crate) fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    bytes.as_deref().map(Bytes::new).serialize(serializer)
}
