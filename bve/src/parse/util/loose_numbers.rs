use serde::de::Visitor;
use serde::export::PhantomData;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct LooseNumber<T>(pub T);

impl<'de, T> Deserialize<'de> for LooseNumber<T>
where
    T: FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LooseFloatVisitor { pd: PhantomData::<T> })
    }
}

struct LooseFloatVisitor<T>
where
    T: FromStr,
{
    pd: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for LooseFloatVisitor<T>
where
    T: FromStr,
{
    type Value = LooseNumber<T>;

    fn expecting<'a>(&self, formatter: &mut Formatter<'a>) -> fmt::Result {
        write!(formatter, "Expected loose float.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut filtered: String = v.chars().filter(|c| !c.is_whitespace()).collect();

        while !filtered.is_empty() {
            let parsed: Result<T, _> = filtered.parse();
            match parsed {
                Ok(v) => return Ok(LooseNumber(v)),
                Err(_) => filtered.pop(),
            };
        }
        Err(serde::de::Error::custom(format!("Error parsing the loose float {}", v)))
    }
}

#[cfg(test)]
mod test {
    use crate::parse::util::LooseNumber;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn loose_number_f32() {
        let l = LooseNumber::<f32>(1.2);
        assert_de_tokens(&l, &[Token::Str("1.2")]);
        assert_de_tokens(&l, &[Token::Str("1.2000000000000")]);
        assert_de_tokens(&l, &[Token::Str("1.2x8")]);
        assert_de_tokens(&l, &[Token::Str("1    .    2")]);
        assert_de_tokens(&l, &[Token::Str("1    .    2 oh yeah!")]);
        let l = LooseNumber::<f32>(1.6E12);
        assert_de_tokens(&l, &[Token::Str("1.6E12")]);
        assert_de_tokens(&l, &[Token::Str("1.6000000000000E12")]);
        assert_de_tokens(&l, &[Token::Str("1.6E12x8")]);
        assert_de_tokens(&l, &[Token::Str("1    .    6         E        12")]);
        assert_de_tokens(&l, &[Token::Str("1    .    6         E        12 oh yeah!")]);
        let l = LooseNumber::<f32>(1.0);
        assert_de_tokens(&l, &[Token::Str("1")]);
        assert_de_tokens(&l, &[Token::Str("1 . ")]);
        assert_de_tokens(&l, &[Token::Str("1 . 0")]);
        assert_de_tokens(&l, &[Token::Str("1 . 0  E  0")]);
        assert_de_tokens(&l, &[Token::Str("1 . 0  E  0 oh yeah!")]);
    }

    #[test]
    fn loose_number_i64() {
        let l = LooseNumber::<i64>(12);
        assert_de_tokens(&l, &[Token::Str("12")]);
        assert_de_tokens(&l, &[Token::Str("1 2")]);
        assert_de_tokens(&l, &[Token::Str("  1  2   ")]);
        assert_de_tokens(&l, &[Token::Str("  +  1  2   ")]);
        assert_de_tokens(&l, &[Token::Str("12 YEAH")]);
        assert_de_tokens(&l, &[Token::Str("12x222222222222")]);
        let l = LooseNumber::<i64>(-2);
        assert_de_tokens(&l, &[Token::Str("-2")]);
        assert_de_tokens(&l, &[Token::Str("-  2 + ")]);
        assert_de_tokens(&l, &[Token::Str("- 2 - ")]);
        assert_de_tokens(&l, &[Token::Str("-2   FUC ")]);
    }
}
