use std::str::FromStr;
use nom::{IResult, combinator::{map_res, recognize, opt}, sequence::preceded, character::complete::digit1, bytes::complete::tag};

pub fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

pub fn parse_numbers(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i32::from_str(s)
    })(input)?;

    Ok((i, number))
}