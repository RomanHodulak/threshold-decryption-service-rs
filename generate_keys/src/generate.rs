use anyhow::anyhow;
use std::io::{Read, Write};

pub fn generate<W: Write>(
    mut private_key: impl Read,
    shares: impl IntoIterator<Item = W>,
    threshold: u8,
) -> Result<(), anyhow::Error> {
    let mut data = String::new();
    private_key.read_to_string(&mut data)?;

    let data = shamir::SecretData::with_secret(&data, threshold);

    for (i, mut share) in shares.into_iter().enumerate() {
        let data = data
            .get_share((i + 1) as u8)
            .map_err(|e| anyhow!("{:?}", e))?;

        share.write_all(&data)?;
    }

    Ok(())
}
