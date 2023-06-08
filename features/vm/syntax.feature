Feature: Syntax
  Scenario Outline: Use literals
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
      | literal                  | result  |
      | ()                       | ()      |
      | 42                       | 42      |

  Scenario: Define a variable
    Given a file named "main.arc" with:
    """
    (let x 42)
    x
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    42
    """

  Scenario: Define variables
    Given a file named "main.arc" with:
    """
    (let x 1)
    (let y 2)
    x
    y
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    1
    2
    """

  Scenario: Overwrite a variable
    Given a file named "main.arc" with:
    """
    (let x 1)
    (let x 2)
    x
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    2
    """
