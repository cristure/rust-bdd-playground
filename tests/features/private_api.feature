Feature: Private APIs

  Scenario: User can retrieve all its open orders
    Given Environment variables "URL", "API_KEY", "PRIVATE_KEY" are set
    When I send a POST request to "/0/private/OpenOrders"
    Then the response status should be 200
    Then the response body should contain 0 errors
