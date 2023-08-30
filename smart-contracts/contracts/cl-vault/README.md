# CL-Vault
## Intro
The CL-vault is a contract that users to move their funds in accordance to given signals by an offchain actor. The user deposits it's funds into the contract. Whenever the range admin submits a new range, the vaults creates a position using that range and deposits all funds in that range.

## Testing
The main effort for testing is done using test tube, this means that any code coverage tools won't pick up on lines covered.
Some functions should still have their own unit tests/prop tests ofcourse, especially if the intended behaviour might not be clear later.
### [Test-tube](https://github.com/osmosis-labs/test-tube)

#### Running tests
To run any test tube tests, a wasm file of the contract needs to be built for test tube. After which the tests can be ran.
To build the wasm file, run `cargo test-tube-build`. after the build succeeds, tests can be ran using `cargo test-tube`. 

#### Writing tests
To make use of our current setup, all tests using test-tube need to go in `src/test_tube`. `test_tube/initialize.rs` contains some helpers to setup 
a Concentrated Liqudity pool and instantiate the cl-vault contract. Any further behaviour tests need to be written potentially in a new file depending on the size of the test

### Unit testing
For unit testing coverage, it is up to the reviewers to decide where that level of granularity is needed, or where just test tube provides adequate testing