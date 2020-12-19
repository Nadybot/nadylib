# nadylib

nadylib is a Rust implementation of the Anarchy Online Chat Protocol using [tokio](https://tokio.rs/).

## Current Status

Done:
* TCP connection
* Login key generation
* Some packet types
* Helper methods for writing and reading the protocol
* MMDB parser

Not yet done:
* Proper implementation of MMDB string formatting
* A lot of packet types
* In the future, an AOBot wrapper with state handling
* Document everything
