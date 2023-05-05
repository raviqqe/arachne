use crate::expression::Expression;

pub fn evaluate(expression: &Expression) -> Expression {
    evaluate_option(expression).unwrap_or_else(nil)
}

fn evaluate_option(expression: &Expression) -> Option<Expression> {
    match expression {
        Expression::Symbol(_) => Some(expression.clone()),
        Expression::Array(array) => match array.as_slice() {
            [] => Some(expression.clone()),
            [predicate, ..] => match predicate {
                Expression::Symbol(symbol) => {
                    let rest = || &array[1..];

                    match symbol.as_str() {
                        "array" => Some(Expression::Array(rest().iter().cloned().collect())),
                        "get" => {
                            let [array, index, ..] = rest() else { return None; };

                            evaluate_array(array)?
                                .get((evaluate_integer(index)? - 1) as usize)
                                .cloned()
                        }
                        "set" => {
                            let [array, index, value, ..] = rest() else { return None; };

                            let mut vector = evaluate_array(array)?.to_vec();

                            if let Some(element) =
                                vector.get_mut((evaluate_integer(index)? - 1) as usize)
                            {
                                *element = value.clone();
                            }

                            Some(Expression::Array(vector))
                        }
                        "len" => {
                            let [array, ..] = rest() else { return None; };

                            Some(Expression::Symbol(format!(
                                "{}",
                                evaluate_array(array)?.len()
                            )))
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
        Expression::Symbol(symbol) => isize::from_str_radix(&symbol, 10).ok(),
        _ => None,
    }
}

fn evaluate_array(expression: &Expression) -> Option<&[Expression]> {
    match expression {
        Expression::Array(array) => Some(&array),
        _ => None,
    }
}

fn nil() -> Expression {
    Expression::Array(vec![])
}
