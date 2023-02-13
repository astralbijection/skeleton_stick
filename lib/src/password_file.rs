use aead::{AeadMut, Key, KeyInit, Nonce};
use aes_gcm::Aes256Gcm;
use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};
use rand_core::{OsRng, RngCore};
use scrypt::scrypt;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// The symbols to detect the file starting.
const FILE_START: &[u8] = b"__SKELETONSTICK\n";

/// The current file format version.
const CURRENT_VERSION: u64 = 1;

/// Number of iterations to run KDF for.
pub const DEFAULT_KDF_ITERS: u32 = 1_000_000;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PasswordEntry {
    name: String,
    password: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileHeader {
    salt: [u8; 32],
    nonce: [u8; 16],
    mac: [u8; 16],
    iters: u32,
    body_length: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PasswordFile {
    entries: Vec<PasswordEntry>,
}

pub struct EncryptParams {
    hash_params: scrypt::Params,
}

pub struct EncryptedPasswordFile {
    ciphertext: Vec<u8>,
    params: EncryptParams,
    nonce: Nonce<Aes256Gcm>,
    salt: [u8; 32],
}

impl PasswordFile {
    pub fn encrypt(
        &self,
        params: EncryptParams,
        password: &[u8],
    ) -> anyhow::Result<EncryptedPasswordFile> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        serde_json::to_writer(&mut encoder, &self)?;
        let plaintext = encoder.finish()?;

        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);

        let mut key: Key<Aes256Gcm> = [0u8; 32].into();
        scrypt(password, &salt, &params.hash_params, &mut key)?;

        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce);

        let mut cipher = Aes256Gcm::new(&key.into());
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())?;

        Ok(EncryptedPasswordFile {
            params,
            ciphertext,
            salt,
            nonce: *nonce,
        })
    }
}

impl EncryptedPasswordFile {
    pub fn decrypt(&self, password: &[u8]) -> anyhow::Result<PasswordFile> {
        let mut key: Key<Aes256Gcm> = [0u8; 32].into();
        scrypt(password, &self.salt, &self.params.hash_params, &mut key)?;

        let mut cipher = Aes256Gcm::new(&key.into());
        let plaintext = cipher.decrypt(&self.nonce, self.ciphertext.as_ref())?;

        let decoder = ZlibDecoder::new(&plaintext[..]);
        let result: PasswordFile = serde_json::from_reader(decoder)?;
        Ok(result)
    }

    pub async fn write(&self, file: &mut (impl AsyncWrite + Unpin)) -> anyhow::Result<()> {
        file.write_all(FILE_START).await?;
        file.write_u64(CURRENT_VERSION).await?;

        file.write_u8(self.params.hash_params.log_n()).await?;
        file.write_u32(self.params.hash_params.r()).await?;
        file.write_u32(self.params.hash_params.p()).await?;

        file.write_all(&self.salt).await?;
        file.write_all(&self.nonce).await?;

        file.write_u64(self.ciphertext.len().try_into()?).await?;
        file.write_all(&self.ciphertext).await?;
        Ok(())
    }

    pub async fn read(file: &mut (impl AsyncRead + Unpin)) -> anyhow::Result<Self> {
        let mut magic = FILE_START.to_owned();
        file.read_exact(&mut magic).await?;
        if magic != FILE_START {
            todo!()
        }

        let version = file.read_u64().await?;
        if version != CURRENT_VERSION {
            todo!()
        }

        let log_n = file.read_u8().await?;
        let r = file.read_u32().await?;
        let p = file.read_u32().await?;
        let params = EncryptParams {
            hash_params: scrypt::Params::new(log_n, r, p)?,
        };

        let mut salt = [0u8; 32];
        file.read_exact(&mut salt).await?;

        let mut nonce: Nonce<Aes256Gcm> = [0u8; 12].into();
        file.read_exact(&mut nonce).await?;

        let body_size: usize = file.read_u64().await?.try_into().unwrap();
        let mut body = vec![0u8; body_size];

        file.read_exact(&mut body).await?;

        Ok(EncryptedPasswordFile {
            ciphertext: body,
            params, nonce, salt
        })
    }
}
