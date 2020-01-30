/*
 * Copyright 2018-2020 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use sawtooth_sdk::signing::{secp256k1, Context};

use super::{Error, Signer};

/// A Sawtooth Secp256k Signer that references a context.
///
/// The SawtoothSecp256k1RefSigner provides an implementation of the Signer trait, that uses a
/// provided Secp256k1Context.
pub struct SawtoothSecp256k1RefSigner<'c> {
    context: &'c secp256k1::Secp256k1Context,
    private_key: secp256k1::Secp256k1PrivateKey,
    public_key: Vec<u8>,
}

impl<'c> SawtoothSecp256k1RefSigner<'c> {
    pub fn new(
        context: &'c secp256k1::Secp256k1Context,
        private_key: secp256k1::Secp256k1PrivateKey,
    ) -> Result<Self, Error> {
        let public_key = context
            .get_public_key(&private_key)
            .map_err(|err| Error::SigningError(format!("Unable to extract public key: {}", err)))?
            .as_slice()
            .to_vec();
        Ok(Self {
            context,
            private_key,
            public_key,
        })
    }
}

impl<'c> Signer for SawtoothSecp256k1RefSigner<'c> {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        self.context
            .sign(message, &self.private_key)
            .map_err(|err| Error::SigningError(format!("Failed to sign message: {}", err)))
            .and_then(|signature| {
                hex::decode(&signature).map_err(|err| {
                    Error::SigningError(format!(
                        "Unable to parse sawtooth signature {} into bytes: {}",
                        signature, err
                    ))
                })
            })
    }

    fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}
