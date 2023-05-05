Feature: Primitives
  Scenario Outline: Create an array
    Given a file named "main.arc" with:
    """
    (array <arguments>)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    <result>
    """

    Examples:
      | arguments | result  |
      |           | ()      |
      | 1         | (1)     |
      | 1 2       | (1 2)   |
      | 1 2 3     | (1 2 3) |
