(let-rec
  fibonacci
  (fn (x)
    (if
      (= x 0) 0
      (= x 1) 1
      (+ (fibonacci (- x 1)) (fibonacci (- x 2))))))

(fibonacci 38)
