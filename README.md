# Staticrypt

The name is an abbreviation of "Static Encryption" - a Rust proc macro library to encrypt text
literals or binary data using AES-256.

The crate is intended to be a successor to [`litcrypt`](https://docs.rs/litcrypt/latest/litcrypt/),
and expand on the overall idea of the library.

Like litcrypt, staticrypt works by encrypting the given data at compile time. In its place, it
leaves the encrypted contents and a 96 bit nonce (unique for every encrypted item), protecting
your data from static analysis tools.

In contrast to to litcrypt's `lc`, staticrypt's `sc` supports all valid Rust string literals,
including those with escape sequences, unicode characters, etc.

To initialize staticrypt in a crate, the `use_staticrypt` macro needs to be called first. See
its doc page for more info on initial setup.

## Example

```rust
use staticrypt::*;

// Needs to be present at the root of the crate (i.e. `main.rs` or `lib.rs`).
use_staticrypt!();

fn main() {
    // Protect sensitive information from static analysis / tampering
    println!("The meaning of life is {}", sc!("42"));
}
```

Everything inside the `sc` macro will be encrypted at compile time. You can verify that none
of the strings are present in cleartext using something like `strings`:

```shell
strings target/debug/my_app | grep 42
```

If the output is blank / does not contain the string you are looking for, then your app is safe
from static analysis tools.

> [!WARNING]
>
> Although using tools like staticrypt makes it very difficult for attackers to view or alter
> your data, it does _not_ make it impossible. You should develop your programs with the
> assumption that a sufficiently determined attacker will be able to reverse engineer your
> encryption and gain access to any data present in your binary, so it is **highly discouraged** to
> use this crate to embed sensitive information like API keys, passwords, private keys etc. in your
> application.
