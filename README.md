# Arachne

## Specification

### Language

#### Quote

```lisp
'(foo bar) ; -> (foo bar)
(quote (foo bar)) ; -> (foo bar)
```

#### Let binding

```lisp
(let foo bar)
```

#### Lambda expression

```lisp
(fn (x y) (+ 42 2045))
```

#### Macro expression

```lisp
(macro (x y) (+ 42 2045))
```

### Standard libraries

```lisp
(use "path/to/module.arc")
```

> WIP

## References

- [rui314/minilisp](https://github.com/rui314/minilisp)
- [The Racket programming language](https://github.com/racket/racket)

## License

[The Unlicense](UNLICENSE)
