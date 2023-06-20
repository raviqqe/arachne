Feature: Algorithm
  Scenario: Define a sum function
    Given a file named "main.arc" with:
    """
    (let-rec
      sum
      (fn (x y)
        (if
          (= x 0) y
          (sum (- x 1) (+ x y)))))

    (sum 1000000)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    500000500000
    """

  Scenario: Define a fibonacci function
    Given a file named "main.arc" with:
    """
    (let-rec
      fibonacci
      (fn (x)
        (if
          (= x 0) 0
          (= x 1) 1
          (+ (fibonacci (- x 1)) (fibonacci (- x 2))))))

    (fibonacci 20)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    6765
    """

  Scenario: Define a tak function
    Given a file named "main.arc" with:
    """
    (let-rec
      tak
      (fn (x y z)
        (if (< y x)
          (tak 
            (tak (- x 1) y z)
            (tak (- y 1) z x)
            (tak (- z 1) x y))
          z)))

    (tak 32 16 8)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    9
    """
