Feature: Array
  Scenario Outline: Use primitive operations
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
      | literal                       | result  |
      | ()                            | ()      |
      | (get () 0)                    | ()      |
      | (set () 0 42)                 | (42)    |
      | (set (set () 0 42) 1 13)      | (42 13) |
      | (= () ())                     | 1       |
      | (= () (set () 0 1))           | ()      |
      | (= (set () 0 1) (set () 0 1)) | 1       |
