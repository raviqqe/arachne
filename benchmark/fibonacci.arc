(let f 
  (fn (x) 
    (if 
      (= x 0) foo
      (= x 1) foo
      0)))

(f x)
