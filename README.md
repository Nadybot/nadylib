# nadylib

nadylib is a Rust implementation of the Anarchy Online Chat Protocol using [tokio](https://tokio.rs/).

## Current Status

Done:

- TCP connection
- Login key generation
- Helper methods for writing and reading the protocol
- MMDB parser
- 100% coverage and fast implementation of MMDB string formatting
- All currently available packet types for chat-only sessions

Not yet done:

- In the future, an AOBot wrapper with state handling
- Examples
