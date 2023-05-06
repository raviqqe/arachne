# Arachne

The programming language for dark mages.

## Syntax

- Atoms (e.g. `42`, `x`)
- Arrays (e.g. `(x y z)`)
- Comments (e.g. `; This is a comment.`)

## Types

- Symbols (e.g. `42`, `x`)
- Arrays (e.g. `(x y z)`)

## Primitives

### Array

```lisp
(array 8 42) ; -> ()
```

Arguments are an element size and capacity.

### Get

```lisp
(get (array 1 42 3) 2) ; -> 42
```

### Set

```lisp
(set (array 1 2 3) 2 42) ; -> (1 42 3)
```

If a value is invalid, the `set` function does nothing and returns the original array.

### Length

```lisp
(len (array 1 2 3)) ; -> 3
```

### Equal

```lisp
(eq 1 1) ; -> 1
(eq 1 2) ; -> 0
```

## Prelude library

### If expression

```lisp
(if
  (condition-1 body-1)
  (condition-2 body-2)
  else-body)
```

### Let binding

```lisp
(let foo bar)
```

### Lambda expression

```lisp
(fn (x y) (+ 42 2045))
```

### Macro expression

```lisp
(macro (x y) (+ 42 2045))
```

### Quote

```lisp
'(foo bar) ; -> (foo bar)
(quote (foo bar)) ; -> (foo bar)
```

### Unquote

```lisp
,foo ; -> 42
(unquote foo) ; -> 42
```

where `foo` is `42`.

### Module import

```lisp
(use "path/to/module.arc")
```

> WIP

## Documentation

- [The core language](core.md)

## References

- [Rust](https://www.rust-lang.org/)
- [Clojure](https://clojure.org/)
- [Racket](https://racket-lang.org/)
- [rui314/minilisp](https://github.com/rui314/minilisp)
- [How many primitives does it take to build a LISP machine? Ten, seven or five?](https://stackoverflow.com/questions/3482389/how-many-primitives-does-it-take-to-build-a-lisp-machine-ten-seven-or-five)

## License

[The Unlicense](UNLICENSE)
