(define (fibonacci x)
  (cond
    [(= x 0) 0]
    [(= x 1) 1]
    [else (+ (fibonacci (- x 1)) (fibonacci (- x 2)))]))

(print (fibonacci 40))
