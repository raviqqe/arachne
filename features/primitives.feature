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
      | elements | index | result |
      |          | 1     | ()     |
      | 1        | 0     | ()     |
      | 1        | 1     | 1      |
      | 1        | 2     | ()     |
      | 1 42     | 2     | 42     |
      | 1 2 42   | 3     | 42     |

  Scenario Outline: Set an element
    Given a file named "main.arc" with:
    """
    (set (array <elements>) <index> <value>)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    <result>
    """

    Examples:
      | elements | index | value | result  |
      |          |       |       | ()      |
      |          | 1     |       | ()      |
      |          | 1     | 42    | (42)    |
      |          | 2     | 42    | (() 42) |
      | 1        | 1     | 42    | (42)    |
      | 1 2      | 2     | 42    | (1 42)  |

  Scenario Outline: Get a length
    Given a file named "main.arc" with:
    """
    (len (array <elements>))
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    <result>
    """

    Examples:
      | elements | result |
      |          | 0      |
      | 1        | 1      |
      | 1 2      | 2      |
      | 1 2 3    | 3      |

  Scenario Outline: Check equality
    Given a file named "main.arc" with:
    """
    (eq <lhs> <rhs>)
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    <result>
    """

    Examples:
      | lhs   | rhs   | result |
      | 0     | 0     | true   |
      | 0     | 1     | false  |
      | 0     | ()    | false  |
      | ()    | ()    | true   |
      | ()    | (1)   | false  |
      | (1)   | (1)   | true   |
      | (1 2) | (1 2) | true   |
