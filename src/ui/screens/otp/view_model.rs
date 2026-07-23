use crate::ui::models::device::DeviceRepo;
use gpui::*;

pub struct OtpViewModel {
    pub device: Entity<DeviceRepo>,
    pub credentials: Vec<OtpCredential>,
    pub loading: bool,
}

#[derive(Clone, Debug)]
pub struct OtpCredential {
    pub name: String,
    pub secret: Vec<u8>,
    pub otp_type: OtpType,
    pub digits: u32,
    pub period: u32,
    pub counter: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OtpType {
    Totp,
    Hotp,
}

pub enum OtpEvent {
    Notification(String),
}

impl EventEmitter<OtpEvent> for OtpViewModel {}

impl OtpViewModel {
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
                        Some(OtpCredential {
                            name: v.get("name")?.as_str()?.to_string(),
                            secret: hex::decode(v.get("secret")?.as_str()?).ok()?,
                            otp_type: match v.get("type")?.as_str()? {
                                "hotp" => OtpType::Hotp,
                                _ => OtpType::Totp,
                            },
                            digits: v.get("digits").and_then(|d| d.as_u64()).unwrap_or(6) as u32,
                            period: v.get("period").and_then(|d| d.as_u64()).unwrap_or(30) as u32,
                            counter: v.get("counter").and_then(|d| d.as_u64()).unwrap_or(0) as u32,
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
                    "type": match c.otp_type { OtpType::Hotp => "hotp", _ => "totp" },
                    "digits": c.digits,
                    "period": c.period,
                    "counter": c.counter,
                })
            })
            .collect();
        std::fs::write(&path, serde_json::to_string_pretty(&items).unwrap_or_default()).is_ok()
    }

    pub fn add_credential(&mut self, cred: OtpCredential) {
        self.credentials.retain(|c| c.name != cred.name);
        self.credentials.push(cred);
        self.save_credentials();
    }

    pub fn remove_credential(&mut self, name: &str) {
        self.credentials.retain(|c| c.name != name);
        self.save_credentials();
    }

    pub fn generate_code(cred: &OtpCredential) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let counter = match cred.otp_type {
            OtpType::Totp => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                now / cred.period as u64
            }
            OtpType::Hotp => cred.counter as u64,
        };
        Self::hmac_code(&cred.secret, counter, cred.digits)
    }

    fn hmac_code(secret: &[u8], counter: u64, digits: u32) -> String {
        use hmac::{Hmac, Mac};
        use sha1::Sha1;

        let counter_bytes = counter.to_be_bytes();
        let mut mac = Hmac::<Sha1>::new_from_slice(secret).unwrap();
        mac.update(&counter_bytes);
        let result = mac.finalize().into_bytes();

        let offset = (result[result.len() - 1] & 0x0F) as usize;
        let truncated = u32::from_be_bytes([
            result[offset],
            result[offset + 1],
            result[offset + 2],
            result[offset + 3],
        ]) & 0x7FFFFFFF;
        let code = truncated % 10u32.pow(digits);
        format!("{:0>width$}", code, width = digits as usize)
    }

    fn cred_file_path() -> std::path::PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("picoforge")
            .join("otp_credentials.json")
    }

    pub fn import_from_qr_image(path: &std::path::Path) -> Result<Vec<OtpCredential>, String> {
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

    pub fn parse_otpauth_uri(uri: &str) -> Option<OtpCredential> {
        let uri = uri.strip_prefix("otpauth://")?;
        let (type_str, rest) = uri.split_once('/')?;
        let otp_type = match type_str {
            "totp" => OtpType::Totp,
            "hotp" => OtpType::Hotp,
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
        let counter = if otp_type == OtpType::Hotp {
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
        Some(OtpCredential {
            name,
            secret,
            otp_type,
            digits,
            period,
            counter,
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
