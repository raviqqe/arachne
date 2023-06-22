Feature: Memory
  @linux
  Scenario Outline: Do not cause any memory error
    When I run the following script:
    """sh
    valgrind --tool=memcheck arachne < bench/<name>/main.arc
    """
    Then the exit status should be 0

    Examples:
      | name      |
      | fibonacci |
      | sum       |
      | tak       |
