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

  Scenario Outline: Get an element
    Given a file named "main.arc" with:
    """
    (get (array <elements>) <index>)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    <result>
    """

    Examples:
      | elements | index | result  |
      |          | 1     | ()      |
      | 1        | 0     | ()      |
      | 1        | 1     | 1       |
      | 1        | 2     | ()      |
      | 1 42     | 2     | 42      |
      | 1 2 42   | 3     | 42      |
