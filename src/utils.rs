use std::borrow::Cow;
use std::env;

pub(crate) fn build_env_opt(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(val) if !val.is_empty() => Some(val),
        _ => None,
    }
}
pub(crate) fn build_env(name: &str) -> String {
    build_env_opt(name).unwrap_or_else(|| {
        let crate_name = std::env!("CARGO_CRATE_NAME").replace('_', "-");
        panic!(
            "Environment variable {name} not defined, are you using {crate_name} outside of build.rs?",
        )
    })
}

macro_rules! define_target_enum {
    (
        $(#[$enum_meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $str:literal
            ),+ $(,)?
        }
        as_str_doc = $as_str_doc:literal,
        from_str_doc = $from_str_doc:literal,
    ) => {
        $(#[$enum_meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                #[doc = concat!("`", $str, "`")]
                $variant,
            )+
            /// Unknown value
            Other(String),
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl $name {
            #[must_use]
            #[doc = $as_str_doc]
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $str,)+
                    Self::Other(s) => s,
                }
            }

            #[doc = $from_str_doc]
            pub fn from_str<'a>(name: impl Into<std::borrow::Cow<'a, str>>) -> Self {
                let name = crate::utils::into_ascii_lowercase(name.into());
                match name.as_ref() {
                    $($str => Self::$variant,)+
                    _ => Self::Other(name.to_string()),
                }
            }
        }
    };
}
pub(crate) use define_target_enum;

/// Equivalent to [`std::str::to_ascii_lowercase`] but does not allocate if the string is already lowercase and
/// reuses the allocation if given a [`Cow::Owned`].
pub fn into_ascii_lowercase(s: Cow<'_, str>) -> Cow<'_, str> {
    match s {
        Cow::Owned(mut s) => {
            s.make_ascii_lowercase();
            Cow::Owned(s)
        }
        Cow::Borrowed(s) => to_ascii_lowercase(s),
    }
}

/// Equivalent to [`std::str::to_ascii_lowercase`] but does not allocate if the string is already lowercase.
pub fn to_ascii_lowercase(s: &'_ str) -> Cow<'_, str> {
    if let Some(first_uppercase) = s
        .bytes()
        .position(|b| b.is_ascii_alphabetic() && !b.is_ascii_lowercase())
    {
        let mut string = String::with_capacity(s.len());
        string.push_str(&s[..first_uppercase]);
        for c in s[first_uppercase..].chars() {
            string.push(c.to_ascii_lowercase());
        }
        Cow::Owned(string)
    } else {
        Cow::Borrowed(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn basic() {
        let test = "Some strIng WITH RaNdOm casing. unicode: ఈ⌛龜∰ēƉʞ‱";
        let expected = test.to_ascii_lowercase();
        let actual = to_ascii_lowercase(test);
        assert_eq!(expected.as_str(), actual.as_ref());
    }

    #[test]
    fn no_alloc_on_lowercase() {
        let test = "some lowercase string. unicode: ఈ⌛龜∰ēƉʞ‱";
        let expected = test.to_ascii_lowercase();
        let actual = to_ascii_lowercase(test);
        assert!(matches!(actual, Cow::Borrowed(_)));
        assert_eq!(expected.as_str(), actual.as_ref());
    }
}
