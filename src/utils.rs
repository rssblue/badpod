pub fn from_str_exact<T: strum::IntoEnumIterator + std::fmt::Display>(s: &str) -> Option<T>
where
{
    T::iter().find(|variant| format!("{variant}") == s)
}

pub fn from_str_case_insensitive<T: strum::IntoEnumIterator + std::fmt::Display>(
    s: &str,
) -> Option<T>
where
{
    T::iter().find(|variant| format!("{variant}").to_lowercase() == s.to_lowercase())
}
