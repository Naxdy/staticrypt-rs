//! # Staticrypt
//!
//! The name is an abbreviation of "Static Encryption" - a Rust proc macro libary to encrypt text
//! literals or binary data using [`Aes256Gcm`].
//!
//! The crate is intended to be a successor to the [`litcrypt`](https://docs.rs/litcrypt/latest/litcrypt/),
//! and expand on the overall idea of the library.
//!
//! Like litcrypt, staticrypt works by encrypting the given data at compile time. In its place, it
//! leaves the encrypted contents and a 96 bit nonce (unique for every encrypted item), protecting
//! your data from static analysis tools.
//!
//! In contrast to to litcrypt's `lc`, staticrypt's [`sc`] supports all valid Rust string literals,
//! including those with escape sequences, unicode characters, etc.
//!
//! To initialize staticrypt in a crate, the [`use_staticrypt`] macro needs to be called first. See
//! its doc page for more info on initial setup.
//!
//! ## Example
//!
//! ```rust
//! use staticrypt::*;
//!
//! // Needs to be present at the root of the crate.
//! use_staticrypt!();
//!
//! fn main() {
//!     // Protect sensitive information from static analysis / tampering
//!     println!("The meaning of life is {}", sc!("42"));
//! }
//! ```
//!
//! Everything inside the [`sc`] macro will be encrypted at compile time. You can verify that none
//! of the strings are present in cleartext using something like `strings`:
//!
//! ```shell
//! strings target/debug/my_app | grep 42
//! ```
//!
//! If the output is blank / does not contain the string you are looking for, then your app is safe
//! from static analysis tools.
//!
//! ## DISCLAIMER
//!
//! Although using tools like staticrypt makes it very difficult for attackers to view or alter
//! your data, it does _not_ make it impossible. You should develop your programs with the
//! assumption that a sufficiently determined attacker will be able to reverse engineer your
//! encryption and gain access to any data present in your binary, so it is **highly discouraged** to
//! use this crate to embed sensitive information like API keys, passwords, private keys etc. in your
//! application.
#![allow(clippy::needless_doctest_main)]

use aes_gcm::{Aes256Gcm, Key, KeyInit, aead::Aead};

/// Initializes the use of staticrypt. Should be used at the top level of a crate, i.e. in your
/// `main.rs` or `lib.rs` (wherever `crate` is pointing to).
///
/// This macro will declare a global const named `STATICRYPT_ENCRYPT_KEY` which contains the key
/// used to decrypt contents encrypted with staticrypt. The key itself is not encrypted.
///
/// The key is derived from the seed stored in the environment variable `STATICRYPT_SEED`. If
/// `STATICRYPT_SEED` is missing, or fewer than 32 characters long, it will be padded with randomly
/// generated bytes until it is of length 32.
///
/// If you desire your builds to be reproducible, set `STATICRYPT_SEED` to contain 32 characters.
/// This way, both the generated key and all nonces will be predictable.
pub use staticrypt_macros::use_staticrypt;

/// Encrypts the contained literal string using [`Aes256Gcm`] with the key embedded using
/// [`use_staticrypt`] and a randomly generated nonce (derived from the `STATICRYPT_SEED` env
/// variable at compile time).
///
/// Example:
///
/// ```rust
/// use staticrypt::*;
///
/// use_staticrypt!();
///
/// fn main() {
///     // "My secret text" will be encrypted in the resulting binary
///     let encrypted = sc!("My secret text");
///     
///     assert_eq!(encrypted, "My secret text");
///     
///     // Also works with unicode / non-standard characters, and escape sequences:
///     let encrypted = sc!("My\0 nonstandard \u{0256} Text \"with escapes\"");
///     
///     assert_eq!(encrypted, "My\0 nonstandard \u{0256} Text \"with escapes\"");
/// }
///
/// ```
pub use staticrypt_macros::sc;

/// Reads and encrypts the specified file contents using [`Aes256Gcm`] with the key embedded using
/// [`use_staticrypt`] and a randomly generated nonce (derived from the `STATICRYPT_SEED` env
/// variable at compile time).
///
/// Note that `sc_bytes` does not do any parsing by default, so it always outputs a
/// [`Vec<u8>`](std::vec::Vec).
///
/// Example:
///
/// ```rust
/// use staticrypt::*;
///
/// use_staticrypt!();
///
/// fn main() {
///     let encrypted = String::from_utf8(sc_bytes!("./testfile.txt"))
///         .expect("Should be valid UTF-8");
///
///     assert_eq!(encrypted, "Hello, staticrypt!\n");
/// }
/// ```
pub use staticrypt_macros::sc_bytes;

use_staticrypt!();

/// Decrypt an input with a given nonce and key using [`Aes256Gcm`].
///
/// Note that manually calling this function should not be necessary, as the [`sc`] macro already
/// does this behind the scenes.
pub fn decrypt(input: &[u8], nonce: &[u8], key: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    cipher
        .decrypt(nonce.into(), input)
        .expect("Failed to decrypt contents")
}

#[cfg(test)]
mod tests {
    use staticrypt_macros::sc;

    #[test]
    fn encrypt_decrypt() {
        let my_text = sc!("super secret woah");

        assert_eq!(my_text, "super secret woah");
    }

    #[test]
    fn nullbyte() {
        let my_text = sc!("sometest\0 withnull");

        assert_eq!(my_text, "sometest\0 withnull");
    }

    #[test]
    fn unicode() {
        let my_text = sc!("I have Unicode \u{0256}");

        assert_eq!(my_text, "I have Unicode \u{0256}");
    }
}
