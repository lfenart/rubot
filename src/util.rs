// returns None if iter.next() == None, panics if iter.next() cannot be parsed as an i8
pub fn try_read<'a, T: std::str::FromStr>(iter: &mut impl Iterator<Item = &'a str>) -> Option<T>
where
    T::Err: std::fmt::Debug,
{
    Some(iter.next()?.parse::<T>().unwrap())
}

pub fn read<'a, T: std::str::FromStr>(iter: &mut impl Iterator<Item = &'a str>) -> T
where
    T::Err: std::fmt::Debug,
{
    try_read(iter).unwrap()
}
