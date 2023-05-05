Feature: Interpreter
  Scenario: Create an array
    Given a file named "main.arc" with:
    """
		(array)
    """
    When I successfully run `turtle < main.arc`
    Then the stdout should contain exactly:
    """
    ()
    """
