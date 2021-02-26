<p align="center">
  <img width="400px" src="logo.svg" />
</p>

![build](https://github.com/nash-io/openlimits/workflows/Rust/badge.svg) ![security](https://github.com/nash-io/openlimits/workflows/Security%20audit/badge.svg)

A open source Rust high performance cryptocurrency trading API with support for multiple exchanges and language wrappers. Focused in safety, correctness and speed.

## Project goals

This is an ambitious project that seeks to create a new industry standard API implementation for secure, correct and high performance cryptocurrency trading. It is initially focused on spot exchanges as a method to define how initial data structures and project architecture should be to allow zero cost abstractions around the exchanges' peculiarities. So initial project goals are:

* Based on Rust, memory safe by default.
* Support for websockets and user defined networking.
* Thin layer wrappers for Java, C#, Python and Node.js
* Easy to add support for additional exchanges.
* Open-source only, now and forever.

Future goals are:

* Support for futures trading
* Support for options trading
* WASM compilation allowing it to be embedded in static web pages

**Warning**: the project is still in development and a lot of breaking changes are being made.


### Community

There is a project slack, [click here](https://join.slack.com/t/openlimitsworkspace/shared_invite/zt-ifhd5tv4-9QL73tCB~fDcRJzDFEuOrQ) to join.

### Requirements (Windows only)

OpenLimits uses `rust_gmp` by default because it runs about 30% faster than `num_bigint`.

If you are on Windows, you need to build and install both `gmp.lib` and `gmp.dll`.
You can find more info about how to build it in the [gmp official page](https://gmplib.org/) and how to install it on Rust in [this comment](https://github.com/ZenGo-X/kms-secp256k1/issues/25#issuecomment-555946633). 

If you want an easier path, you can use `num_bigint` instead, because it doesn't require any of the previous steps. To do it, you can use the library as:

`openlimits = { version = "0.1", default-features = false, features = ["num_bigint"] }`

### Testing

In order to run the tests you will have to provide environment variables for the sandbox API of the exchanges, you can use environment variables or use a `.env` file.

### Sponsorship

We invite industry participants to join us in sponsoring a new high quality open source standard for crypto trading APIs. [Nash](https://nash.io) is dedicating a maintainer and initial rewards for external contributors that close issues. Look for the wiki [rewards table](https://github.com/nash-io/openlimits/wiki/Rewards-sizes) and for reward size labels on open issues.