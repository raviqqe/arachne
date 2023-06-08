Feature: VM interpreter
  Scenario: Use a number
    Given a file named "main.arc" with:
    """
    42
    """
    When I run `arachne` interactively
    And I pipe in the file "main.arc"
    Then the exit status should be 0
