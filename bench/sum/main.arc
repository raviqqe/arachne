(let-rec
  sum
  (fn (x y)
    (if
      (= x 0) y
      (sum (- x 1) (+ x y)))))

(sum 100000000)
