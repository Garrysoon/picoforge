use crate::ui::models::device::DeviceRepo;
use gpui::*;

pub struct OathViewModel {
    pub device: Entity<DeviceRepo>,
    pub credentials: Vec<OathCredential>,
    pub loading: bool,
}

#[derive(Clone, Debug)]
pub struct OathCredential {
    pub name: String,
    pub secret: Vec<u8>,
    pub cred_type: OathType,
    pub digits: u32,
    pub period: u32,
    pub counter: u32,
    pub algorithm: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OathType {
    Totp,
    Hotp,
}

pub enum OathEvent {
    Notification(String),
}

impl EventEmitter<OathEvent> for OathViewModel {}

impl OathViewModel {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, models: &crate::ui::app::AppModels) -> Self {
        Self {
            device: models.device.clone(),
            credentials: Vec::new(),
            loading: false,
        }
    }

    pub fn load_credentials(&mut self) {
        let path = Self::cred_file_path();
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(items) = serde_json::from_str::<Vec<serde_json::Value>>(&data) {
                self.credentials = items
                    .into_iter()
                    .filter_map(|v| {
                        Some(OathCredential {
                            name: v.get("name")?.as_str()?.to_string(),
                            secret: hex::decode(v.get("secret")?.as_str()?).ok()?,
                            cred_type: match v.get("type")?.as_str()? {
                                "hotp" => OathType::Hotp,
                                _ => OathType::Totp,
                            },
                            digits: v.get("digits").and_then(|d| d.as_u64()).unwrap_or(6) as u32,
                            period: v.get("period").and_then(|d| d.as_u64()).unwrap_or(30) as u32,
                            counter: v.get("counter").and_then(|d| d.as_u64()).unwrap_or(0) as u32,
                            algorithm: v
                                .get("algorithm")
                                .and_then(|d| d.as_str())
                                .unwrap_or("sha1")
                                .to_string(),
                        })
                    })
                    .collect();
            }
        }
    }

    pub fn save_credentials(&self) -> bool {
        let path = Self::cred_file_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let items: Vec<serde_json::Value> = self
            .credentials
            .iter()
            .map(|c| {
                serde_json::json!({
                    "name": c.name,
                    "secret": hex::encode(&c.secret),
                    "type": match c.cred_type { OathType::Hotp => "hotp", _ => "totp" },
                    "digits": c.digits,
                    "period": c.period,
                    "counter": c.counter,
                    "algorithm": c.algorithm,
                })
            })
            .collect();
        std::fs::write(&path, serde_json::to_string_pretty(&items).unwrap_or_default()).is_ok()
    }

    pub fn add_credential(&mut self, cred: OathCredential) {
        self.credentials.retain(|c| c.name != cred.name);
        self.credentials.push(cred);
        self.save_credentials();
    }

    pub fn remove_credential(&mut self, name: &str) {
        self.credentials.retain(|c| c.name != name);
        self.save_credentials();
    }

    pub fn generate_code(cred: &OathCredential) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let counter = match cred.cred_type {
            OathType::Totp => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                now / cred.period as u64
            }
            OathType::Hotp => cred.counter as u64,
        };
        Self::hmac_code(&cred.secret, counter, cred.digits, &cred.algorithm)
    }

    fn hmac_code(secret: &[u8], counter: u64, digits: u32, algorithm: &str) -> String {
        use hmac::{Hmac, Mac};
        use sha1::Sha1;
        use sha2::{Sha256, Sha512};

        let counter_bytes = counter.to_be_bytes();
        let mac_result: Vec<u8> = match algorithm {
            "sha256" => {
                let mut mac = Hmac::<Sha256>::new_from_slice(secret).unwrap();
                mac.update(&counter_bytes);
                mac.finalize().into_bytes().to_vec()
            }
            "sha512" => {
                let mut mac = Hmac::<Sha512>::new_from_slice(secret).unwrap();
                mac.update(&counter_bytes);
                mac.finalize().into_bytes().to_vec()
            }
            _ => {
                let mut mac = Hmac::<Sha1>::new_from_slice(secret).unwrap();
                mac.update(&counter_bytes);
                mac.finalize().into_bytes().to_vec()
            }
        };

        let offset = (mac_result[mac_result.len() - 1] & 0x0F) as usize;
        let truncated = u32::from_be_bytes([
            mac_result[offset],
            mac_result[offset + 1],
            mac_result[offset + 2],
            mac_result[offset + 3],
        ]) & 0x7FFFFFFF;
        let code = truncated % 10u32.pow(digits);
        format!("{:0>width$}", code, width = digits as usize)
    }

    fn cred_file_path() -> std::path::PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("picoforge")
            .join("oath_credentials.json")
    }

    pub fn import_from_qr_image(path: &std::path::Path) -> Result<Vec<OathCredential>, String> {
        let img = image::open(path).map_err(|e| format!("Failed to open image: {}", e))?;
        let gray = img.to_luma8();
        let mut prepared = rqrr::PreparedImage::prepare(gray);
        let grids = prepared.detect_grids();
        let mut creds = Vec::new();
        for grid in grids {
            if let Ok((_, content)) = grid.decode() {
                if let Some(cred) = Self::parse_otpauth_uri(&content) {
                    creds.push(cred);
                }
            }
        }
        if creds.is_empty() {
            return Err("No valid otpauth:// URIs found in QR code".into());
        }
        Ok(creds)
    }

    pub fn parse_otpauth_uri(uri: &str) -> Option<OathCredential> {
        let uri = uri.strip_prefix("otpauth://")?;
        let (type_str, rest) = uri.split_once('/')?;
        let cred_type = match type_str {
            "totp" => OathType::Totp,
            "hotp" => OathType::Hotp,
            _ => return None,
        };
        let (label, query) = rest.split_once('?').unwrap_or((rest, ""));
        let label = urlencoding::decode(label).ok()?.to_string();
        let mut params: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        for pair in query.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                params.insert(k.to_string(), urlencoding::decode(v).ok()?.to_string());
            }
        }
        let secret_b32 = params.get("secret")?;
        let secret = base32_decode_local(secret_b32)?;
        let digits = params
            .get("digits")
            .and_then(|d| d.parse::<u32>().ok())
            .unwrap_or(6);
        let period = params
            .get("period")
            .and_then(|d| d.parse::<u32>().ok())
            .unwrap_or(30);
        let algorithm = params
            .get("algorithm")
            .map(|a| a.to_lowercase())
            .unwrap_or_else(|| "sha1".to_string());
        let counter = if cred_type == OathType::Hotp {
            params
                .get("counter")
                .and_then(|c| c.parse::<u32>().ok())
                .unwrap_or(0)
        } else {
            0
        };
        let name = if let Some(issuer) = params.get("issuer") {
            format!("{}:{}", issuer, label)
        } else {
            label
        };
        Some(OathCredential {
            name,
            secret,
            cred_type,
            digits,
            period,
            counter,
            algorithm,
        })
    }
}

fn base32_decode_local(s: &str) -> Option<Vec<u8>> {
    use base64::Engine;
    let s = s.trim_end_matches('=');
    let mut padded = s.to_string();
    let padding = (8 - padded.len() % 8) % 8;
    padded.push_str(&"=".repeat(padding));
    base64::engine::general_purpose::STANDARD
        .decode(padded.as_bytes())
        .ok()
}
