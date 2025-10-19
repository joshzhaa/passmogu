`passmogu` builds a library that implements most of a password manager. It can be paired with different "frontends".
`passmogu-cli` builds a binary. This implements the CLI frontend of `passmogu`

## Implementaton
A "vault" is saved as a string somewhere accessible to the "frontend" (e.g. on the filesystem) and entirely loaded
into memory.
