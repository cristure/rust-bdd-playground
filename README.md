# rust-bdd-playground

BDD scaffold with rust-cucumber. The feature files are located under `./tests/features`

## Pre-requirements
1. Rust
2. Cargo
3. Docker

## Test execution

#### Cargo
One could trigger the test with cargo. Note that the test expects 3 environment variables to be set. Namely `URL`, `API_KEY`, `PRIVATE_KEY`
```
export URL=<YOUR_URL>
export API_KEY=<YOUR_API_KEY>
export PRIVATE_KEY=<YOUR_PRIVATE_KEY>
cargo test
```

#### Docker
One could trigger the tests with docker as well. 

1. Build the docker image.
```
docker build . -t rust-bdd-playground
```

2. Run the tests
```
docker run -e URL=<YOUR_URL> \
 -e API_KEY=<YOUR_API_KEY> \ 
 -e PRIVATE_KEY=<YOUR_PRIVATE_KEY> \
  rust-bdd-playground
```
