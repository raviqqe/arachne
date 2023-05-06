# Core language

## Primitives

### Get

```lisp
(get xs 0) ; -> 42
```

### Set

```lisp
(set xs 1 42) ; -> (1 42 3)
```

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
