use std::{path::Path, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::utils::{AppError, AppResult};

const SCAN_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Debug, PartialEq)]
pub enum ScanResult {
    Clean,
    /// the signature name reported by clamd, e.g. `Heuristics.Encrypted.Zip`
    Infected(String),
}

/// Streams a file to clamd via `INSTREAM`. Anything other than a clear verdict (connection
/// problems, timeouts, size limit errors) is returned as an error so callers can fail closed.
pub async fn scan_file(address: &str, path: &Path) -> AppResult<ScanResult> {
    tokio::time::timeout(SCAN_TIMEOUT, scan_file_inner(address, path))
        .await
        .map_err(|_| AppError::Anyhow(anyhow::anyhow!("clamd scan timed out")))?
}

async fn scan_file_inner(address: &str, path: &Path) -> AppResult<ScanResult> {
    let mut stream = TcpStream::connect(address).await?;
    stream.write_all(b"zINSTREAM\0").await?;
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = vec![0u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        stream.write_all(&(read as u32).to_be_bytes()).await?;
        stream.write_all(&buffer[..read]).await?;
    }
    stream.write_all(&0u32.to_be_bytes()).await?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;
    parse_response(&String::from_utf8_lossy(&response))
}

fn parse_response(response: &str) -> AppResult<ScanResult> {
    let response = response.trim_end_matches(['\0', '\n']).trim();
    let verdict = response.strip_prefix("stream:").map(str::trim);
    match verdict {
        Some("OK") => Ok(ScanResult::Clean),
        Some(v) if v.ends_with(" FOUND") => Ok(ScanResult::Infected(
            v.trim_end_matches(" FOUND").to_string(),
        )),
        _ => Err(AppError::Anyhow(anyhow::anyhow!(
            "unexpected clamd response: {response}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_clamd_responses() {
        assert_eq!(parse_response("stream: OK\0").unwrap(), ScanResult::Clean);
        assert_eq!(
            parse_response("stream: Win.Test.EICAR_HDB-1 FOUND\0").unwrap(),
            ScanResult::Infected("Win.Test.EICAR_HDB-1".to_string())
        );
        assert_eq!(
            parse_response("stream: Heuristics.Encrypted.Zip FOUND\0").unwrap(),
            ScanResult::Infected("Heuristics.Encrypted.Zip".to_string())
        );
        assert!(parse_response("INSTREAM size limit exceeded. ERROR\0").is_err());
        assert!(parse_response("stream: lstat() failed. ERROR\0").is_err());
        assert!(parse_response("").is_err());
    }
}
