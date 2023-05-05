# Arachne

## Syntax

- Atoms (e.g. `true`, `42`, `x`)
- Arrays (e.g. `(x y z)`)
- Comments (e.g. `; This is a comment.`)

## Types

- Symbols (e.g. `true`, `42`, `x`)
- Arrays (e.g. `(x y z)`)

## Primitives

### Array

```lisp
(array 1 2) ; -> (1 2)
```

### Get

```lisp
(get (array 1 42 3) 2) ; -> 42
```

### Set

```lisp
(set (array 1 2 3) 2 42) ; -> (1 42 3)
```

### Length

```lisp
(len (array 1 2 3)) ; -> 3
```

### Equal

```lisp
(eq 1 1) ; -> true
(eq 1 2) ; -> false
```

## Prelude library

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

### Module import

```lisp
(use "path/to/module.arc")
```

> WIP

## References

- [Rust programming language](https://www.rust-lang.org/)
- [Racket programming language](https://racket-lang.org/)
- [rui314/minilisp](https://github.com/rui314/minilisp)
- [How many primitives does it take to build a LISP machine? Ten, seven or five?](https://stackoverflow.com/questions/3482389/how-many-primitives-does-it-take-to-build-a-lisp-machine-ten-seven-or-five)

## License

[The Unlicense](UNLICENSE)
