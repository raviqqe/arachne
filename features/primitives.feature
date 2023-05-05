Feature: Primitives
  Scenario: Create an array
    Given a file named "main.arc" with:
    """
		(array)
    """
		When I successfully run `arachne` interactively
		And I pipe in the file "main.arc"
    Then the stdout should contain exactly:
    """
    ()
    """
