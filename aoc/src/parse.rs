pub fn parse_comma_separated<T>(text: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    text.split(",").map(|s| s.parse::<T>().unwrap()).collect()
}
