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

### Get

```lisp
(get xs 0) ; -> 42
```

### Set

```lisp
(set xs 1 42) ; -> (1 42 3)
```

If a value is invalid, the `set` function does nothing and returns the original array.

### Length

```lisp
(len xs) ; -> 42
```

### Equal

```lisp
(eq 0 ()) ; -> 1
(eq 0 0) ; -> 1
(eq 0 1) ; -> 0
```

## Prelude library

### If expression

```lisp
(if
  condition-0 then-body-0
  condition-1 then-body-1
  else-body)
```

### Let binding

```lisp
(let foo bar)
```

#### Self recursion

```lisp
(let-rec foo (fn () (foo)))
```

#### Mutual recursion

```lisp
(let-rec
  foo (fn () (bar))
  bar (fn () (foo)))
```

### Lambda expression

```lisp
(fn (x y) (+ x y))
```

### Macro expression

```lisp
(macro (x y) `(let ,x ,y))
```

### Quote

- Quasi-quotation

```lisp
`(foo bar) ; -> (foo bar)
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

## Design notes

- [The core language](core.md)

## References

- [Rust](https://www.rust-lang.org/)
- [Clojure](https://clojure.org/)
- [Racket](https://racket-lang.org/)
- [rui314/minilisp](https://github.com/rui314/minilisp)
- [How many primitives does it take to build a LISP machine? Ten, seven or five?](https://stackoverflow.com/questions/3482389/how-many-primitives-does-it-take-to-build-a-lisp-machine-ten-seven-or-five)

## License

[The Unlicense](UNLICENSE)
