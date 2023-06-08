Feature: Arithmetic
  Scenario Outline: Use arithmetic operations
    Given a file named "main.arc" with:
    """
    <literal>
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    <result>
    """

    Examples:
      | literal  | result |
      | (+ 1 2)  | 3      |
      | (- 2 1)  | 1      |
      | (* 2 3)  | 6      |
      | (/ 6 2)  | 3      |
