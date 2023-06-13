Feature: Arithmetic
  Scenario Outline: Use arithmetic operations
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
      | function                   | expression  | result |
      | (fn () 42)                 | (f)         | 42     |
      | (fn (x) 42)                | (f 0)       | 42     |
      | (fn (x) x)                 | (f 42)      | 42     |
      | (fn (x y) (+ x y))         | (f 42 2045) | 2087   |
      | (fn (x y z) (+ (* x y) z)) | (f 42 2 7)  | 91     |
