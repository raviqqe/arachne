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

  Scenario Outline: Use boolean operations
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
      | literal   | result |
      | (not 0)   | 1      |
      | (not 1)   | ()     |
      | (and 0 1) | ()     |
      | (and 1 1) | 1      |
      | (and 1 2) | 2      |
      | (or 0 0)  | ()     |
      | (or 0 1)  | 1      |
      | (or 0 2)  | 2      |
