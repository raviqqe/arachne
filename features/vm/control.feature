Feature: Control flow
  Scenario Outline: Use an if expression
    Given a file named "main.arc" with:
    """
    <expression>
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    <result>
    """

    Examples:
      | expression        | result |
      | (if 0 13 42)      | 42     |
      | (if 1 42 13)      | 42     |
      | (if 0 42)         | ()     |
      | (if 0 13 1 42 13) | 42     |
