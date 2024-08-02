Feature: Public APIs

  Scenario: User can retrieve server time
    Given Environment variable "URL" is set
    When I send a GET request to "/0/public/Time"
    Then the response status should be 200
    Then the response body should contain 0 errors

  Scenario: User can retrieve information about trading
    Given Environment variable "URL" is set
    When I send a GET request to "/0/public/AssetPairs?pair=XBTUSD"
    Then the response status should be 200
    Then the response body should contain 0 errors

