Feature: Number
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
      | literal | result  |
      | ()      | ()      |
      | 42      | 42      |

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
      | literal | result |
      | (+ 1 2) | 3      |
      | (- 2 1) | 1      |
      | (* 2 3) | 6      |
      | (/ 6 2) | 3      |

  Scenario Outline: Use comparison operations
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
      | literal | result |
      | (= 1 1) | 1      |
      | (= 0 1) | ()     |
      | (< 0 0) | ()     |
      | (< 0 1) | 1      |
