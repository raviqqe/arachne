use crate::expression::Expression;

pub fn evaluate(expression: &Expression) -> Expression {
    evaluate_option(expression).unwrap_or_else(nil)
}

fn evaluate_option(expression: &Expression) -> Option<Expression> {
    match expression {
        Expression::Symbol(_) => Some(expression.clone()),
        Expression::Array(array) => match array.as_slice() {
            [] => None,
            [predicate, ..] => match evaluate(predicate) {
                Expression::Symbol(symbol) => {
                    let rest = || &array[1..];
                    let arguments = rest().iter().map(evaluate).collect::<Vec<_>>();

                    match symbol.as_str() {
                        "array" => Some(rest().to_vec().into()),
                        "eq" => Some((arguments.get(0)? == arguments.get(1)?).to_string().into()),
                        "get" => evaluate_array(arguments.get(0)?)?
                            .get((evaluate_integer(arguments.get(1)?)? - 1) as usize)
                            .cloned(),
                        "set" => {
                            let mut vector = evaluate_array(arguments.get(0)?)?.to_vec();
                            let index = (evaluate_integer(arguments.get(1)?)? - 1) as usize;

                            if index >= vector.len() {
                                vector.extend((0..index + 1 - vector.len()).map(|_| nil()));
                            }

                            vector[index] = arguments.get(2)?.clone();

                            Some(vector.into())
                        }
                        "len" => {
                            Some(format!("{}", evaluate_array(arguments.get(0)?)?.len()).into())
                        }
                        _ => None,
                    }
                }
                Expression::Array(_) => None,
            },
        },
    }
}

fn evaluate_integer(expression: &Expression) -> Option<isize> {
    match expression {
        Expression::Symbol(symbol) => symbol.parse::<isize>().ok(),
        _ => None,
    }
}

fn evaluate_array(expression: &Expression) -> Option<&[Expression]> {
    match expression {
        Expression::Array(array) => Some(array),
        _ => None,
    }
}

fn nil() -> Expression {
    vec![].into()
}

#[cfg(test)]
mod tests {
    use super::evaluate;
    use pretty_assertions::assert_eq;

    #[test]
    fn evaluate_symbol() {
        let expression = "foo".into();

        assert_eq!(evaluate(&expression), expression);
    }

    mod array {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn evaluate_empty() {
            assert_eq!(evaluate(&vec!["array".into()].into()), vec![].into());
        }

        #[test]
        fn evaluate_element() {
            assert_eq!(
                evaluate(&vec!["array".into(), "1".into()].into()),
                vec!["1".into()].into()
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
                        "1".into()
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
                        "2".into()
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
                        "1".into(),
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
                        "3".into(),
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
