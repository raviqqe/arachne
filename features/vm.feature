Feature: VM interpreter
  Scenario Outline: Literal
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
      | literal | result  |
      | ()      | ()      |
      | 42      | 42      |
