# Arachne

## Language

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

### Get

```lisp
(get x 2) ; -> 42
```

where `x` is `'(1 42 3)`

### Set

```lisp
(set x 2 42) ; -> (1 42 3)
```

where `x` is `'(1 2 3)`

### Comment

```lisp
; This is a comment.
```

## Prelude library

### Module import

```lisp
(use "path/to/module.arc")
```

> WIP

## References

- [rui314/minilisp](https://github.com/rui314/minilisp)
- [The Racket programming language](https://github.com/racket/racket)
- [How many primitives does it take to build a LISP machine? Ten, seven or five?](https://stackoverflow.com/questions/3482389/how-many-primitives-does-it-take-to-build-a-lisp-machine-ten-seven-or-five)

## License

[The Unlicense](UNLICENSE)
