//! Matrix device identifiers.

#[cfg(feature = "rand")]
use crate::generate_localpart;

/// A Matrix key ID.
///
/// Device identifiers in Matrix are completely opaque character sequences. This type is provided
/// simply for its semantic value.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent, crate = "serde"))]
pub struct DeviceId(str);

opaque_identifier!(DeviceId, DeviceIdBox, "device ID");

impl DeviceId {
    /// Generates a random `DeviceId`, suitable for assignment to a new device.
    #[cfg(feature = "rand")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
    pub fn new() -> Box<Self> {
        Self::from_owned(generate_localpart(8))
    }
}

impl AsRef<str> for Box<DeviceId> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for &'a DeviceId {
    fn from(s: &'a str) -> Self {
        DeviceId::from_borrowed(s)
    }
}

impl From<&str> for Box<DeviceId> {
    fn from(s: &str) -> Self {
        DeviceId::from_owned(s.into())
    }
}

impl From<String> for Box<DeviceId> {
    fn from(s: String) -> Self {
        DeviceId::from_owned(s.into())
    }
}

impl From<Box<DeviceId>> for String {
    fn from(id: Box<DeviceId>) -> Self {
        id.into_owned().into()
    }
}

impl Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Box<DeviceId> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::deserialize_id(deserializer, "An IP address or hostname")
    }
}

partial_eq_string!(DeviceId);
partial_eq_string!(Box<DeviceId>);

/// Generates a random `DeviceId`, suitable for assignment to a new device.
#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
#[deprecated = "use DeviceId::new instead"]
pub fn generate() -> Box<DeviceId> {
    DeviceId::new()
}

#[cfg(all(test, feature = "rand"))]
mod tests {
    use super::DeviceId;

    #[test]
    fn generate_device_id() {
        assert_eq!(DeviceId::new().as_str().len(), 8);
    }
}
