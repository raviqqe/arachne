Feature: Function
  Scenario Outline: Call a function
    Given a file named "main.arc" with:
    """
    (let f <function>)
    <expression>
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    <result>
    """

    Examples:
      | function                             | expression  | result |
      | (fn () 42)                           | (f)         | 42     |
      | (fn (x) 42)                          | (f 0)       | 42     |
      | (fn (x) x)                           | (f 42)      | 42     |
      | (fn (x y) (+ x y))                   | (f 42 2045) | 2087   |
      | (fn (x y z) (+ (* x y) z))           | (f 42 2 7)  | 91     |
      | (fn (x) (let y x) y)                 | (f 42)      | 42     |
      | (fn (x) (let y x) (let z (+ x y)) z) | (f 42)      | 84     |
      | (fn (x) (+ x x))                     | (f (f 2))   | 8      |

  Scenario: Define a recursive function
    Given a file named "main.arc" with:
    """
    (let-rec
      f
      (fn (x)
        (if (= x 0)
          42
          (f (- x 1)))))
    (f 3)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    42
    """

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

  Scenario: Create a closure
    Given a file named "main.arc" with:
    """
    (let x 42)
    (let f (fn () x))

    (f)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    42
    """
