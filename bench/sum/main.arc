(let-rec
  sum
  (fn (x y) 
    (if 
      (= x 0) y
      (sum y -1))))

(sum 1000000)
