# Core language

## Primitives

### Array

```lisp
(array) ; -> ()
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
(eq 1 1) ; -> 1
(eq 1 2) ; -> 0
```
