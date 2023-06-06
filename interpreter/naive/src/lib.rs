mod error;

use async_stream::try_stream;
use error::InterpretError;
use futures::{Stream, StreamExt};
use once_cell::sync::Lazy;
use runtime::{Array, Symbol, Value, NIL};
use std::error::Error;

static ARRAY: Lazy<Symbol> = Lazy::new(|| "array".into());
static EQ: Lazy<Symbol> = Lazy::new(|| "eq".into());
static GET: Lazy<Symbol> = Lazy::new(|| "get".into());
static SET: Lazy<Symbol> = Lazy::new(|| "set".into());
static LEN: Lazy<Symbol> = Lazy::new(|| "len".into());

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
    (|| {
        if let Some(mut array) = value.as_array().cloned() {
            for index in 0..array.len_usize() {
                let value = array.get_usize(index);
                array = array.set_usize(index, evaluate(value));
            }

            if let Some(symbol) = array.get_usize(0).to_symbol() {
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
    })()
    .unwrap_or(NIL)
}

#[cfg(test)]
mod tests {
    use super::evaluate;
    use pretty_assertions::assert_eq;
    use runtime::{Value, NIL};

    #[test]
    fn evaluate_symbol() {
        let value = Value::from("foo");

        assert_eq!(evaluate(value.clone()), value);
    }

    mod array {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn evaluate_empty() {
            assert_eq!(evaluate(["array".into()].into()), NIL);
        }

        #[test]
        fn evaluate_element() {
            assert_eq!(
                evaluate(["array".into(), 1.0.into()].into()),
                [1.0.into()].into()
            );
        }

        #[test]
        fn evaluate_elements() {
            assert_eq!(
                evaluate(["array".into(), 1.0.into(), 2.0.into()].into()),
                [1.0.into(), 2.0.into()].into()
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
                    [
                        "get".into(),
                        ["array".into(), "42".into()].into(),
                        0.0.into()
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
                    [
                        "get".into(),
                        ["array".into(), "42".into()].into(),
                        1.0.into()
                    ]
                    .into()
                ),
                NIL
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
                    [
                        "set".into(),
                        ["array".into(), 0.0.into()].into(),
                        0.0.into(),
                        "42".into()
                    ]
                    .into()
                ),
                ["42".into()].into(),
            );
        }

        #[test]
        fn set_element_out_of_bounds() {
            assert_eq!(
                evaluate(
                    [
                        "set".into(),
                        ["array".into(), 0.0.into()].into(),
                        2.0.into(),
                        "42".into()
                    ]
                    .into()
                ),
                [0.0.into(), [].into(), "42".into()].into(),
            );
        }
    }

    mod len {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn get_len_of_zero() {
            assert_eq!(
                evaluate(["len".into(), ["array".into()].into(),].into()),
                0.0.into(),
            );
        }

        #[test]
        fn get_len_of_one() {
            assert_eq!(
                evaluate(["len".into(), ["array".into(), 1.0.into()].into(),].into()),
                1.0.into(),
            );
        }

        #[test]
        fn get_len_of_two() {
            assert_eq!(
                evaluate(
                    [
                        "len".into(),
                        ["array".into(), 1.0.into(), 2.0.into()].into(),
                    ]
                    .into()
                ),
                2.0.into(),
            );
        }
    }

    mod eq {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn check_equal_symbols() {
            assert_eq!(
                evaluate(["eq".into(), 0.0.into(), 0.0.into()].into()),
                1.0.into(),
            );
        }

        #[test]
        fn check_symbols_not_equal() {
            assert_eq!(evaluate(["eq".into(), 0.0.into(), 1.0.into()].into()), NIL);
        }

        #[test]
        fn check_equal_arrays() {
            assert_eq!(
                evaluate(
                    [
                        "eq".into(),
                        ["array".into()].into(),
                        ["array".into()].into(),
                    ]
                    .into()
                ),
                1.0.into(),
            );
        }

        #[test]
        fn check_arrays_not_equal() {
            assert_eq!(
                evaluate(
                    [
                        "eq".into(),
                        ["array".into()].into(),
                        ["array".into(), 1.0.into()].into(),
                    ]
                    .into()
                ),
                NIL,
            );
        }
    }
}
