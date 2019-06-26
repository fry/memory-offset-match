use regex::bytes::Regex;

use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n, take_while1, is_a},
  character::complete::{alpha1, hex_digit1, multispace0},
  branch::alt,
  combinator::{map, map_parser, all_consuming},
  sequence::{separated_pair, delimited, preceded},
  multi::separated_list
};

fn capture_pattern(input: &str) -> IResult<&str, &str> {
    is_a("?")(input)
}

fn capture_inner(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        alpha1,
        tag(":"),
        preceded(
            multispace0,
            capture_pattern)
    )(input)
}

fn capture(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        tag("("),
        capture_inner,
        tag(")")
    )(input)
}

fn regex_capture(input: (&str, &str)) -> String {
    format!(r"(?P<{}>.{{{}}})", input.0, input.1.len())
}

fn pattern_token(input: &str) -> IResult<&str, String> {
    alt((
        map(
            hex_digit1,
            regex_hex),
        map(
            tag("?"),
            |_: &str| ".".to_string()),
        map(
            capture,
            regex_capture)
    ))(input)
}

fn regex_hex(input: &str) -> String {
    format!(r"\x{}", input)
}

fn byte_pattern_regex_string(input: &str) -> IResult<&str, String> {
    let (input, tokens) = all_consuming(separated_list(
        tag(" "),
        pattern_token,
    ))(input)?;

    let regex = format!("(?-u){}", tokens.join(""));
    Ok((input, regex))
}

pub fn valid_byte_pattern_regex(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_, regex_str) = byte_pattern_regex_string(input).map_err(|_| "Failed to parse byte string")?;
    Regex::new(&regex_str)?;
    Ok(regex_str)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_pattern() {
        let regex = byte_pattern_regex_string("55 89 F7 48 89 FB E8 ? ? ? ? 84 C0 74 ? 8B 53 08");
        assert_eq!(regex.unwrap().1, r"(?-u)\x55\x89\xF7\x48\x89\xFB\xE8....\x84\xC0\x74.\x8B\x53\x08");
    }

    #[test]
    fn simple_pattern_regex() {
        valid_byte_pattern_regex("55 89 F7 48 89 FB E8 ? ? ? ? 84 C0 74 ? 8B 53 08").unwrap();
    }

    #[test]
    fn capture_inner_str() {
        let capture = capture_inner("helloworld: ????");
        let res = capture.unwrap().1;
        assert_eq!(res.0, "helloworld");
        assert_eq!(res.1, "????");
    }

    #[test]
    fn capture_str() {
        let capture_res = capture("(helloworld: ????)");
        let res = capture_res.unwrap().1;
        assert_eq!(res.0, "helloworld");
        assert_eq!(res.1, "????");
    }

    #[test]
    fn capture_pattern() {
        let regex = byte_pattern_regex_string("55 (lol: ????) ? 08");
        assert_eq!(regex.unwrap().1, r"(?-u)\x55(?P<lol>.{4}).\x08");
    }

    #[test]
    #[should_panic]
    fn invalid_capture_pattern() {
        byte_pattern_regex_string("55 (: ????) ? 08").unwrap();
    }
}
