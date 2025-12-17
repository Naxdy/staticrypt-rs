use std::sync::LazyLock;

use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit,
    aead::{Aead, OsRng, rand_core::RngCore},
};
use parking_lot::Mutex;
use proc_macro_crate::crate_name;
use proc_macro_error2::abort;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use rand::SeedableRng;
use rand::prelude::StdRng;
use syn::Ident;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeedError {
    #[error("STATICRYPT_SEED must be at most 32 characters long, but is {0} characters long")]
    InvalidLength(usize),
}

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Key is {0} characters long, when it should be exactly 32")]
    InvalidKeyLength(usize),
}

static RNG: LazyLock<Mutex<StdRng>> = LazyLock::new(|| {
    let seed = get_seed().unwrap_or_else(|e| {
        panic!("Failed to define global RNG variable: {e}");
    });

    let mut arg = [0; 32];
    arg.copy_from_slice(&seed);

    Mutex::new(StdRng::from_seed(arg))
});

pub fn gen_decrypt_quote(contents: &[u8], is_string: bool) -> TokenStream {
    let key = get_key();

    let (encrypted, nonce) = match encrypt(contents, &key) {
        Ok(e) => e,
        Err(e) => {
            abort! {
                Span::call_site(),
                "Could not encrypt data: {}", e
            }
        }
    };

    let encrypted_literal = byte_array_literal(&encrypted);

    let nonce_literal = byte_array_literal(&nonce);

    let crate_name = staticrypt_crate_name();

    let d_quote = if is_string {
        quote! {
            ::std::string::String::from_utf8(#crate_name::decrypt(ENCRYPTED, NONCE, crate::STATICRYPT_ENCRYPT_KEY)).unwrap()
        }
    } else {
        quote! {
            #crate_name::decrypt(ENCRYPTED, NONCE, crate::STATICRYPT_ENCRYPT_KEY)
        }
    };

    quote! {
        {
            const ENCRYPTED: &[u8] = &#encrypted_literal;
            const NONCE: &[u8] = &#nonce_literal;

            #d_quote
        }
    }
}

pub fn init() -> TokenStream {
    let key = get_key();
    let key_literal = byte_array_literal(&key);

    quote! {
        const STATICRYPT_ENCRYPT_KEY: &[u8] = &#key_literal;
    }
}

pub fn staticrypt_crate_name() -> TokenStream {
    match crate_name("staticrypt") {
        Ok(r) => match r {
            proc_macro_crate::FoundCrate::Itself => quote! {crate},
            proc_macro_crate::FoundCrate::Name(name) => {
                let name = Ident::new(&name, Span::call_site());
                quote! {::#name}
            }
        },
        Err(e) => {
            abort! {
                Span::call_site(),
                "Error occurred while trying to determine crate name: {}", e
            }
        }
    }
}

/// Encrypts a byte input, returns a tuple in the form of (encrypted, nonce).
pub fn encrypt(input: &[u8], key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), EncryptionError> {
    if key.len() != 32 {
        return Err(EncryptionError::InvalidKeyLength(key.len()));
    }

    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(&mut *RNG.lock());

    let ciphertext = cipher
        .encrypt(&nonce, input)
        .expect("Failed to encrypt input");

    Ok((ciphertext, nonce.to_vec()))
}

pub fn get_key() -> Vec<u8> {
    static ENCRYPT_KEY: LazyLock<Vec<u8>> =
        LazyLock::new(|| Aes256Gcm::generate_key(&mut *RNG.lock()).to_vec());

    ENCRYPT_KEY.clone()
}

fn get_seed() -> Result<Vec<u8>, SeedError> {
    static RANDOM_SEED: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let mut out = vec![0; 32];

        let mut rng = OsRng;
        rng.fill_bytes(&mut out);

        out
    });

    let mut seed: Vec<u8> = std::env::var("STATICRYPT_SEED")
        .map(|e| e.into())
        .unwrap_or(RANDOM_SEED.to_vec());

    if seed.len() > 32 {
        return Err(SeedError::InvalidLength(seed.len()));
    }

    for i in seed.len()..32 {
        seed.push(RANDOM_SEED[i]);
    }

    Ok(seed)
}

pub fn byte_array_literal(input: &[u8]) -> TokenStream {
    quote! {
        [
            #(
                #input
            ),*
        ]
    }
}
