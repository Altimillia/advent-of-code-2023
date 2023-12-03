use std::str::FromStr;
use nom::{IResult, combinator::{map_res, recognize, opt}, sequence::preceded, character::complete::digit1, bytes::complete::tag};
#[allow(dead_code)]
pub fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

pub fn parse_numbers(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i32::from_str(s)
    })(input)?;

    Ok((i, number))
}

pub fn usize_to_i32(num: usize) -> Result<i32, String> {
    // Check if the `usize` value can fit within the range of `i32`
    if num > i32::MAX as usize {
        return Err(format!("Invalid input. The value {} is too large to fit within the range of i32.", num));
    }

    // Convert the `usize` value to `i32`
    let converted_num = num as i32;

    Ok(converted_num)
}