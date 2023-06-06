mod error;

use async_stream::try_stream;
use error::InterpretError;
use futures::{Stream, StreamExt};
use once_cell::sync::Lazy;
use runtime::{Array, Symbol, Value, NIL};
use std::error::Error;

static ARRAY: Lazy<Symbol> = Lazy::new(|| Symbol::from("array"));
static EQ: Lazy<Symbol> = Lazy::new(|| Symbol::from("eq"));
static GET: Lazy<Symbol> = Lazy::new(|| Symbol::from("get"));
static SET: Lazy<Symbol> = Lazy::new(|| Symbol::from("set"));
static LEN: Lazy<Symbol> = Lazy::new(|| Symbol::from("len"));

pub fn interpret<E: Error + 'static>(
    values: &mut (impl Stream<Item = Result<Value, E>> + Unpin),
) -> impl Stream<Item = Result<Value, InterpretError>> + '_ {
    try_stream! {
        while let Some(result) = values.next().await {
            yield evaluate(result.map_err(|error| InterpretError::Other(error.into()))?);
        }
    }
}

fn evaluate(value: Value) -> Value {
    evaluate_option(value).unwrap_or(NIL)
}

fn evaluate_option(value: Value) -> Option<Value> {
    if let Some(array) = value.as_array() {
        if let Some(symbol) = evaluate(array.get_usize(0)).to_symbol() {
            if symbol == *ARRAY {
                let len = array.len_usize();
                let mut new = Array::new(len - 1);

                for index in 1..len {
                    new = new.set_usize(index - 1, array.get_usize(index));
                }

                Some(new.into())
            } else if symbol == *EQ {
                Some(((array.get_usize(1) == array.get_usize(2)) as usize as f64).into())
            } else if symbol == *GET {
                Some(array.get_usize(1).as_array()?.get(array.get_usize(2)))
            } else if symbol == *SET {
                Some(
                    array
                        .get_usize(1)
                        .as_array()?
                        .clone()
                        .set(array.get_usize(2), array.get_usize(3))
                        .into(),
                )
            } else if symbol == *LEN {
                Some(array.get_usize(1).as_array()?.len().into())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate;
    use pretty_assertions::assert_eq;
    use runtime::Symbol;
    use runtime::{Array, NIL};

    #[test]
    fn evaluate_symbol() {
        let expression = Symbol::from("foo").into();

        assert_eq!(evaluate(expression), expression);
    }

    mod array {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn evaluate_empty() {
            assert_eq!(
                evaluate(Array::from([Symbol::from("array").into()]).into()),
                NIL
            );
        }

        #[test]
        fn evaluate_element() {
            assert_eq!(
                evaluate(["array".into(), 1.0.into()].into()),
                ["1".into()].into()
            );
        }

        #[test]
        fn evaluate_elements() {
            assert_eq!(
                evaluate(&vec!["array".into(), "1".into(), "2".into()].into()),
                vec!["1".into(), "2".into()].into()
            );
        }
    }

    mod get {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn get_element() {
            assert_eq!(
                evaluate(
                    &vec![
                        "get".into(),
                        vec!["array".into(), "42".into()].into(),
                        "0".into()
                    ]
                    .into()
                ),
                "42".into()
            );
        }

        #[test]
        fn get_element_out_of_bounds() {
            assert_eq!(
                evaluate(
                    &vec![
                        "get".into(),
                        vec!["array".into(), "42".into()].into(),
                        "1".into()
                    ]
                    .into()
                ),
                vec![].into()
            );
        }
    }

    mod set {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn set_element() {
            assert_eq!(
                evaluate(
                    &vec![
                        "set".into(),
                        vec!["array".into(), "0".into()].into(),
                        "0".into(),
                        "42".into()
                    ]
                    .into()
                ),
                vec!["42".into()].into(),
            );
        }

        #[test]
        fn set_element_out_of_bounds() {
            assert_eq!(
                evaluate(
                    &vec![
                        "set".into(),
                        vec!["array".into(), "0".into()].into(),
                        "2".into(),
                        "42".into()
                    ]
                    .into()
                ),
                vec!["0".into(), vec![].into(), "42".into()].into(),
            );
        }
    }

    mod len {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn get_len_of_zero() {
            assert_eq!(
                evaluate(&vec!["len".into(), vec!["array".into()].into(),].into()),
                "0".into(),
            );
        }

        #[test]
        fn get_len_of_one() {
            assert_eq!(
                evaluate(&vec!["len".into(), vec!["array".into(), "1".into()].into(),].into()),
                "1".into(),
            );
        }

        #[test]
        fn get_len_of_two() {
            assert_eq!(
                evaluate(
                    &vec![
                        "len".into(),
                        vec!["array".into(), "1".into(), "2".into()].into(),
                    ]
                    .into()
                ),
                "2".into(),
            );
        }
    }

    mod eq {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn check_equal_symbols() {
            assert_eq!(
                evaluate(&vec!["eq".into(), "0".into(), "0".into()].into()),
                "true".into(),
            );
        }

        #[test]
        fn check_symbols_not_equal() {
            assert_eq!(
                evaluate(&vec!["eq".into(), "0".into(), "1".into()].into()),
                "false".into(),
            );
        }

        #[test]
        fn check_equal_arrays() {
            assert_eq!(
                evaluate(
                    &vec![
                        "eq".into(),
                        vec!["array".into()].into(),
                        vec!["array".into()].into(),
                    ]
                    .into()
                ),
                "true".into(),
            );
        }

        #[test]
        fn check_arrays_not_equal() {
            assert_eq!(
                evaluate(
                    &vec![
                        "eq".into(),
                        vec!["array".into()].into(),
                        vec!["array".into(), "1".into()].into(),
                    ]
                    .into()
                ),
                "false".into(),
            );
        }
    }
}
