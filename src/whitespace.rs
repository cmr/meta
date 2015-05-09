use read_token;
use range::Range;

use {
    ParseError,
    Parameter,
    Rule,
    Select,
    Token,
};

/// Stores information about whitespace.
pub struct Whitespace {
    /// Whether the whitespace is optional or required.
    pub optional: bool,
}

impl Whitespace {
    /// Parse whitespace.
    /// If whitespace is required and no whitespace is found,
    /// an error will be reported.
    pub fn parse(&self, chars: &[char], offset: usize) ->
        Result<Range, (Range, ParseError)>
    {
        let range = read_token::whitespace(chars, offset);
        if range.length == 0 && !self.optional {
            Err((range, ParseError::ExpectedWhitespace))
        } else {
            Ok(range)
        }
    }
}

/// A hard coded parameter rule for whitespace.
pub static WHITESPACE: Parameter<'static> = Parameter {
    name: "whitespace",
    args: &["optional"],
    value: None,
    body: &[
        Rule::Whitespace(Whitespace { optional: true }),
        Rule::Token(Token {
            text: "whitespace",
            inverted: None,
            property: None
        }),
        Rule::Select(Select { args: &[
            Rule::Token(Token {
                text: "?",
                inverted: Some(false),
                property: Some("optional"),
            }),
            Rule::Token(Token {
                text: "!",
                inverted: Some(true),
                property: Some("optional"),
            }),
        ]}),
    ],
};

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;

    #[test]
    fn optional() {
        let text = "a,b, c";
        let chars: Vec<char> = text.chars().collect();
        let optional_whitespace = Whitespace { optional: true };
        assert_eq!(optional_whitespace.parse(&chars, 0),
            Ok(Range::new(0, 0)));
        assert_eq!(optional_whitespace.parse(&chars[4..], 4),
            Ok(Range::new(4, 1)));
    }

    #[test]
    fn required() {
        let text = "a,   b,c";
        let chars: Vec<char> = text.chars().collect();
        let required_whitespace = Whitespace { optional: false };
        assert_eq!(required_whitespace.parse(&chars[2..], 2),
            Ok(Range::new(2, 3)));
        // Prints an error message to standard error output.
        assert_eq!(required_whitespace.parse(&chars[7..], 7),
            Err((Range::new(7, 0), ParseError::ExpectedWhitespace)));
    }
}
