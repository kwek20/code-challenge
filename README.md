## Goal
Takes in CSV of transactions,
runs throuhg a transactions engine
output the client account balances

Run the binary: `cargo run -- transactions.csv > accounts.csv`

## Build info
clippy requires cargo nightly to run
`cargo +nightly clippy`

fmt also requires cargo nightly to run for some of the selected features
`cargo +nightly clippy`

Running the example requires either a passed argument for the input file, or set in the .env during development build. 
Release build doesnt use .env

## Environment variables
Environment variables read from dotenvy are only active in debug mode.
An example file is placed in `.env.example` and can be renamed to `.env`.

We read the following:

Input file is used in debug mode, when no parameter has been passed.
Resource folder is not related to this, hence it being prepended here.

`INPUT_FILE="resources/transactions.csv"`

Resource folder is used when a parameter is passed to the binary

`RESOURCE_FOLDER="resources"`

## Logging
Due to the requirements of the challenge outputting the result to stdout, we log informative messages to stderr. 

## Testing
Tests were done by ingesting larger transaction samples and verifying manually, and using a secondary AI, what the output should be.
Unit tests could specifically be added to the `Account` struct modification functions for improvement, and on the general client  process() function. 