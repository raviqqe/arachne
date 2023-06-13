(let f 
  (fn (x) 
    (if 
      (= x 0) 0
      (= x 1) 1
      (+ (f (- x 1)) (f (- x 2))))))

(f x)
