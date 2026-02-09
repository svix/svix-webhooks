// SPDX-FileCopyrightText: © 2026 Svix Authors
// SPDX-License-Identifier: MIT
//
use std::{
    borrow::Cow,
    collections::{hash_map::IntoValues, HashMap},
};

use bytes::Bytes;
use http::{header::InvalidHeaderName, HeaderMap, HeaderName, HeaderValue};

/// A map from String -> T that preserves insertion case,
/// but doesn't allow duplicates (that is to say, inserting
/// "foo" -> "bar", and "Foo" -> "baz" will only result in one key:
/// "Foo" -> "baz"
///
/// Intended use case is really for storing headers (hence the name)
#[derive(Debug, Clone)]
pub struct CasePreservingHeaderMap<T = HeaderValue> {
    /// We implement this as a map from normalized key to (real key, value).
    ///
    /// There are other ways to do this -- we could keep the real keys in
    /// a second map, or we could keep them in a vec SwissTable style, or
    /// we could define a new CaseInsensitiveString wrapper type that
    /// overrides PartialEq, Ord, and Hash, but I think this is the simplest
    inner: HashMap<HeaderName, (String, T)>,
}

impl<T> Default for CasePreservingHeaderMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AsOptionalHeaderName {
    fn as_optional_header_name(&self) -> Option<Cow<'_, HeaderName>>;
}

impl AsOptionalHeaderName for HeaderName {
    fn as_optional_header_name(&self) -> Option<Cow<'_, HeaderName>> {
        Some(Cow::Borrowed(self))
    }
}

impl AsOptionalHeaderName for str {
    fn as_optional_header_name(&self) -> Option<Cow<'_, HeaderName>> {
        Some(Cow::Owned(HeaderName::from_bytes(self.as_bytes()).ok()?))
    }
}

impl<T> CasePreservingHeaderMap<T> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Try to insert the given string into the map as a header. Will fail if it is an invalid header name.
    ///
    /// Intentionally takes a key that is AsRef<str> + Into<String>
    /// so that you can avoid copies if you already have a string, but
    /// also avoid calling `.to_owned()` at every call-site if you have
    /// &str.
    pub fn try_insert<K: AsRef<str> + Into<String>>(
        &mut self,
        key: K,
        value: T,
    ) -> Result<(), InvalidHeaderName> {
        let bytes = key.as_ref().as_bytes();
        let normalized = HeaderName::from_bytes(bytes)?;
        let original = key.into();
        self.inner.insert(normalized, (original, value));
        Ok(())
    }

    /// Insert the given pre-validated HeaderName and value to the map.
    ///
    /// The original string that's stored will be the normalized (i.e., lower-case) version of the HeaderName
    pub fn insert(&mut self, key: HeaderName, value: T) {
        let original = key.to_string();
        self.inner.insert(key, (original, value));
    }

    /// Remove the given key from this map, ignoring its case.
    ///
    /// Returns the contained value, if there was one.
    pub fn remove<K: AsOptionalHeaderName + ?Sized>(&mut self, key: &K) -> Option<T> {
        let key = key.as_optional_header_name()?;
        self.inner.remove(key.as_ref()).map(|v| v.1)
    }

    /// Get the value associated with the given (normalized) key
    pub fn get<K: AsOptionalHeaderName + ?Sized>(&self, key: &K) -> Option<&T> {
        let key = key.as_optional_header_name()?;
        self.inner.get(key.as_ref()).map(|v| &v.1)
    }

    /// Get the non-normalized form of the given key
    pub fn get_key<K: AsOptionalHeaderName + ?Sized>(&self, key: &K) -> Option<&str> {
        let key = key.as_optional_header_name()?;
        self.inner.get(key.as_ref()).map(|v| v.0.as_str())
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn contains_key<K: AsOptionalHeaderName + ?Sized>(&self, key: &K) -> bool {
        let Some(key) = key.as_optional_header_name() else {
            return false;
        };
        self.inner.contains_key(key.as_ref())
    }

    /// Iterate over all key-value pairs. The key is returned in
    /// non-normalized (original) form
    pub fn iter(&self) -> impl Iterator<Item = &(String, T)> {
        self.inner.values()
    }

    /// Convert into two maps: one from HeaderName -> T, and one from HeaderName -> Raw Name
    pub fn into_maps(self) -> (HeaderMap<T>, HeaderMap<Bytes>) {
        let mut value_map = HeaderMap::with_capacity(self.len());
        let mut name_map = HeaderMap::with_capacity(self.len());
        for (key, (raw_key, value)) in self.inner.into_iter() {
            value_map.insert(key.clone(), value);
            name_map.insert(key, Bytes::from(raw_key.into_bytes()));
        }
        (value_map, name_map)
    }

    /// Iterate over all keys. The key is returned in
    /// non-normalized (original) form
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.inner.values().map(|v| v.0.as_str())
    }

    /// Iterate over all values. The key is not included.
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.inner.values().map(|v| &v.1)
    }
}

impl<T> IntoIterator for CasePreservingHeaderMap<T> {
    type Item = (String, T);
    // TODO: Once TAIT lands, remove this ugly hardcoded iterator
    // type
    type IntoIter = IntoValues<HeaderName, (String, T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_values()
    }
}

impl<T: Clone + PartialEq> PartialEq<Self> for CasePreservingHeaderMap<T> {
    // this is terrible but it's only used for tests
    fn eq(&self, other: &Self) -> bool {
        self.iter().cloned().collect::<HashMap<String, T>>()
            == other.iter().cloned().collect::<HashMap<String, T>>()
    }
}

impl<T: Clone + PartialEq> Eq for CasePreservingHeaderMap<T> {}

impl<'a, T> std::ops::Index<&'a str> for CasePreservingHeaderMap<T> {
    type Output = T;

    fn index(&self, index: &'a str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, T> std::ops::Index<&'a HeaderName> for CasePreservingHeaderMap<T> {
    type Output = T;

    fn index(&self, index: &'a HeaderName) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl From<CasePreservingHeaderMap<HeaderValue>> for HashMap<String, HeaderValue> {
    fn from(value: CasePreservingHeaderMap<HeaderValue>) -> Self {
        value.into_iter().collect()
    }
}

impl From<CasePreservingHeaderMap<HeaderValue>> for HeaderMap {
    fn from(value: CasePreservingHeaderMap<HeaderValue>) -> Self {
        let mut map = HeaderMap::new();
        for (k, (_, v)) in value.inner.into_iter() {
            map.insert(k, v);
        }
        map
    }
}

impl<V> FromIterator<(HeaderName, V)> for CasePreservingHeaderMap<V> {
    fn from_iter<T: IntoIterator<Item = (HeaderName, V)>>(iter: T) -> CasePreservingHeaderMap<V> {
        let mut map = CasePreservingHeaderMap::new();
        map.extend(iter);
        map
    }
}

impl<V> Extend<(HeaderName, V)> for CasePreservingHeaderMap<V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (HeaderName, V)>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        self.inner.reserve(iter.size_hint().0);
        iter.for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }
}

impl<V, const N: usize> From<[(HeaderName, V); N]> for CasePreservingHeaderMap<V> {
    fn from(arr: [(HeaderName, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderMap, HeaderName, HeaderValue,
    };
    use maplit::hashset;

    use super::CasePreservingHeaderMap;

    #[test]
    fn test_basic() {
        let mut map = CasePreservingHeaderMap::new();
        map.try_insert("foo", HeaderValue::from_static("bar"))
            .unwrap();
        assert_eq!(map.get("foo"), Some(&HeaderValue::from_static("bar")));
        assert_eq!(map.get("bar"), None);
        assert_eq!(map.get("FOO"), Some(&HeaderValue::from_static("bar")));

        map.try_insert("Foo", HeaderValue::from_static("baz"))
            .unwrap();
        assert_eq!(map.get("foo"), Some(&HeaderValue::from_static("baz")));
        assert_eq!(map.get("Foo"), Some(&HeaderValue::from_static("baz")));
        assert_eq!(map.get_key("foo"), Some("Foo"));
        map.try_insert("bar", HeaderValue::from_static("bing"))
            .unwrap();

        assert_eq!(map.remove("Foo"), Some(HeaderValue::from_static("baz")));
        assert_eq!(map.get("foo"), None);
        assert_eq!(map.get("bar"), Some(&HeaderValue::from_static("bing")));

        map.try_insert("foo\nbar", HeaderValue::from_static("garbage"))
            .unwrap_err();

        let example = HeaderName::from_static("example");
        map.insert(example.clone(), HeaderValue::from_static("example"));
        map.try_insert("EXAMPLE", HeaderValue::from_static("EXAMPLE"))
            .unwrap();
        assert_eq!(map["example"], HeaderValue::from_static("EXAMPLE"));
        assert_eq!(map[&example], HeaderValue::from_static("EXAMPLE"));
    }

    #[test]
    fn test_into_headermap() {
        let mut map = CasePreservingHeaderMap::new();
        map.insert(ACCEPT, HeaderValue::from_static("bar"));
        map.insert(CONTENT_TYPE, HeaderValue::from_static("bar"));

        let converted = HeaderMap::from(map);
        assert_eq!(converted.len(), 2);
    }

    #[test]
    fn test_iterators() {
        let mut map = CasePreservingHeaderMap::<&'static str>::new();
        map.try_insert("foo", "vfoo").unwrap();
        map.try_insert("bar", "vbar").unwrap();
        map.try_insert("Bar", "vBar").unwrap();
        map.try_insert("BAZ", "vBAZ").unwrap();

        assert_eq!(
            map.keys().collect::<HashSet<_>>(),
            hashset! {"foo", "Bar", "BAZ"}
        );
        assert_eq!(
            map.values().collect::<HashSet<&&str>>(),
            hashset! {&"vfoo", &"vBar", &"vBAZ"}
        );

        assert_eq!(
            map.iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect::<HashSet<_>>(),
            hashset! {
                ("foo".to_string(), "vfoo"),
                ("Bar".to_string(), "vBar"),
                ("BAZ".to_string(), "vBAZ")
            }
        );

        assert_eq!(
            map.into_iter().collect::<HashSet<_>>(),
            hashset! {
                ("foo".to_string(), "vfoo"),
                ("Bar".to_string(), "vBar"),
                ("BAZ".to_string(), "vBAZ")
            }
        );
    }

    #[test]
    fn test_from_array() {
        let map = CasePreservingHeaderMap::from([(AUTHORIZATION, "auth"), (CONTENT_TYPE, "ct")]);
        assert_eq!(map.len(), 2);
        assert_eq!(map[&AUTHORIZATION], "auth");
        assert_eq!(map[&CONTENT_TYPE], "ct");
    }
}
