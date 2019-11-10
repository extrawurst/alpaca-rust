## rest-api
Welcome. `#[api]` allows you to create a rest-api by simply annotating a trait.
Don't expect much to work yet.

## goals
Support as many flavours and special cases out there in different Rest APIs with as little as possible effort.

## why all this magic?

* Modularity: Having an API simply implement a Trait allows to swap out the remote API against some internal one easily.
* Focus: Because working against an API should not distract from what you actually want to do *with* that API.
* Stability: Hand written API bindings are harder to maintain and easier to introduce bugs
* Testability: It also simplifies writing your application in a test driven approach. Simply use `#[mock]`([see mock crate](https://github.com/carlosdp/mock_derive)) and run against a mocked API

## todo

- [x] proof of concept
- [ ] string results
- [ ] define base url in attribute
- [ ] support url parameters
- [ ] support query parameters
- [ ] support post using json
- [ ] allow serializable types in api