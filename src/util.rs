// returns None if iter.next() == None, panics if iter.next() cannot be parsed as an i8
pub fn try_read_i8<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<i8> {
    Some(iter.next()?.parse::<i8>().unwrap())
}

pub fn read_i8<'a>(iter: &mut impl Iterator<Item = &'a str>) -> i8 {
    try_read_i8(iter).unwrap()
}
