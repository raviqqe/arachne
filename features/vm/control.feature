Feature: Control flow
  Scenario Outline: Call a function
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
      | expression   | result |
      | (if 0 42 13) | 42     |
      | (if 0 42)    | ()     |
