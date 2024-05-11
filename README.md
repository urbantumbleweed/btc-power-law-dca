# Bitcoin Power Law DCA

Bitcoin Power Law DCA is a simple application designed to help investors optimize their dollar cost averaging (DCA) strategy for Bitcoin investments. By comparing the current market price of Bitcoin to an estimated price derived from the Power Law, the application calculates a multiplier. This multiplier can be used to adjust the amount of Bitcoin to purchase in a DCA approach, potentially allowing for more strategic investment decisions based on long-term price trends.

Credits to Giovanni Santostasi for the discovery of the Bitcoin Power Law. The foundational information necessary to develop this application was inspired by his work. For more details, visit [Giovanni Santostasi's article on the Bitcoin Power Law](https://giovannisantostasi.medium.com/the-bitcoin-power-law-theory-962dfaf99ee9).


## Feature tracking

- [ ] Create configuration file for server settings
- [ ] Create a wrapper module for accessing configuration
- [ ] Write to log file log.txt on server error/crash
- [ ] Automatially restart the server if it crashes or encounters an error
- [ ] Set up automatic testing
- [ ] Set up CI/CD pipeline to deploy to AWS as a Docker image serverless function
- [ ] Basic HTTPS Server implementation
- [ ] Request handling for DCA amount
- [ ] Bitcoin price retrieval method
- [ ] Power Law calculation algorithm
- [ ] Date math functions
- [ ] Response generation for clients
    - [ ] Return a 200 OK response to the client if the request is valid and the calculation is successful
    - [ ] Return a 200 OK response to the client with the calculated multiplier if the request is valid and the calculation is successful
    - [ ] Return a 200 OK response to the client with the calculated multiplier and adjusted DCA amount if the request is valid and the calculation is successful
- [ ] Error handling mechanisms
    - [ ] Log errors to log.txt
    - [ ] Return a 500 Internal Server Error response to the client if an error occurs
    - [ ] Return a 400 Bad Request response to the client if an invalid request is made

## Project Goals

This is a passion project for learning Rust and Bitcoin with the best practices as outlined in The Pragmatic Programmer. This application will require the following minimum functionality:

1. **HTTPS Server**: A secure server implemented in Rust that can handle incoming HTTPS requests.

2. **Request Handling**: Ability to parse incoming requests with a DCA dollar amount parameter.

3. **Bitcoin Price Retrieval**: A method to fetch the current market price of Bitcoin from a reliable source, with minimal dependencies.

4. **Power Law Calculation**: Implement the Power Law algorithm to estimate the Bitcoin price and calculate the multiplier based on the current market price.

5. **Date Math**: Functions to handle simple date calculations necessary for the Power Law algorithm or for logging purposes.

6. **Response Generation**: After calculating the adjusted dollar amount and the multiplier, the server should send this data back to the client in a secure and structured format.

7. **Error Handling**: Proper error handling for scenarios such as invalid requests, failure to fetch Bitcoin price, or any calculation errors.

8. **Configuration Driven**: The application should be configurable, allowing for easy adjustments to parameters such as the Power Law constants, API endpoints for Bitcoin price retrieval, and server settings.

9. **Actor/Process Model**: Build the application using an actor/process model to handle concurrent requests efficiently and safely.

10. **Testing and Assertions**: The application should be fully tested with runtime assertions to ensure the correctness of the calculations and the reliability of the server under various conditions.


