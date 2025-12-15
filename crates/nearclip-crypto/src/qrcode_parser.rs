//! 二维码解析
//!
//! 提供二维码解析功能，用于扫描配对二维码。
//!
//! # Example
//!
//! ```ignore
//! use nearclip_crypto::{QrCodeParser, QrCodeGenerator, PairingData};
//!
//! // 生成二维码
//! let data = PairingData::new("device-id".to_string(), &[0x04; 65]);
//! let generator = QrCodeGenerator::new();
//! let png = generator.generate_png(&data).unwrap();
//!
//! // 解析二维码
//! let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();
//! assert_eq!(parsed.device_id, "device-id");
//! ```

use crate::{CryptoError, PairingData};
use image::ImageReader;
use rqrr::PreparedImage;
use std::io::Cursor;
use tracing::{debug, instrument, warn};

/// 二维码解析器
///
/// 用于解析配对二维码中的内容。所有方法都是静态的，
/// 无需实例化即可使用。
///
/// # Example
///
/// ```ignore
/// use nearclip_crypto::QrCodeParser;
///
/// let png_data: Vec<u8> = /* QR code PNG bytes */;
/// let content = QrCodeParser::parse_from_bytes(&png_data).unwrap();
/// ```
pub struct QrCodeParser;

impl QrCodeParser {
    /// 从 PNG 图片字节解析二维码内容
    ///
    /// # Arguments
    ///
    /// * `png_data` - PNG 格式的二维码图片字节
    ///
    /// # Returns
    ///
    /// 二维码中编码的文本内容
    ///
    /// # Errors
    ///
    /// 返回 `CryptoError::QrCodeParsing` 如果：
    /// - 图片格式无效
    /// - 无法解码图片
    /// - 未找到二维码
    /// - 二维码解码失败
    #[instrument(skip(png_data), fields(data_len = png_data.len()))]
    pub fn parse_from_bytes(png_data: &[u8]) -> Result<String, CryptoError> {
        // 1. 解码 PNG 图片
        let img = ImageReader::new(Cursor::new(png_data))
            .with_guessed_format()
            .map_err(|e| {
                warn!("Invalid image format: {}", e);
                CryptoError::QrCodeParsing(format!("Invalid image format: {}", e))
            })?
            .decode()
            .map_err(|e| {
                warn!("Failed to decode image: {}", e);
                CryptoError::QrCodeParsing(format!("Failed to decode image: {}", e))
            })?;

        // 2. 转换为灰度图
        let gray = img.to_luma8();
        debug!(
            "Converted image to grayscale: {}x{}",
            gray.width(),
            gray.height()
        );

        // 3. 准备二维码检测
        let mut prepared = PreparedImage::prepare(gray);

        // 4. 检测和解码二维码
        let grids = prepared.detect_grids();
        if grids.is_empty() {
            warn!("No QR code found in image");
            return Err(CryptoError::QrCodeParsing("No QR code found".to_string()));
        }

        debug!("Found {} QR code(s) in image", grids.len());

        // 5. 解码第一个检测到的二维码
        let (_, content) = grids[0].decode().map_err(|e| {
            warn!("Failed to decode QR code: {}", e);
            CryptoError::QrCodeParsing(format!("Failed to decode QR code: {}", e))
        })?;

        debug!("QR code decoded, content length: {}", content.len());
        Ok(content)
    }

    /// 从 PNG 图片字节解析配对数据
    ///
    /// 这是一个便捷方法，结合了二维码解析和 JSON 解析。
    ///
    /// # Arguments
    ///
    /// * `png_data` - PNG 格式的二维码图片字节
    ///
    /// # Returns
    ///
    /// 解析后的配对数据
    ///
    /// # Errors
    ///
    /// 返回错误如果：
    /// - 二维码解析失败 (`CryptoError::QrCodeParsing`)
    /// - JSON 解析失败 (`CryptoError::JsonSerialization`)
    /// - 数据验证失败 (`CryptoError::InvalidPairingData`)
    #[instrument(skip(png_data), fields(data_len = png_data.len()))]
    pub fn parse_pairing_data(png_data: &[u8]) -> Result<PairingData, CryptoError> {
        let json = Self::parse_from_bytes(png_data)?;
        debug!("Parsing JSON content from QR code");

        let data = PairingData::from_json(&json)?;
        data.validate()?;

        debug!("Successfully parsed pairing data for device: {}", data.device_id);
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ConnectionInfo, EcdhKeyPair, QrCodeGenerator};

    #[test]
    fn test_parse_from_bytes_valid_qrcode() {
        // 生成有效的二维码
        let generator = QrCodeGenerator::new();
        let data = PairingData::new("test-device".to_string(), &[0x04; 65]);
        let png = generator.generate_png(&data).unwrap();

        // 解析
        let content = QrCodeParser::parse_from_bytes(&png).unwrap();
        assert!(content.contains("test-device"));
        assert!(content.contains("version"));
    }

    #[test]
    fn test_parse_pairing_data_roundtrip() {
        let keypair = EcdhKeyPair::generate();
        let original = PairingData::new("my-device".to_string(), &keypair.public_key_bytes())
            .with_connection_info(
                ConnectionInfo::new()
                    .with_ip("192.168.1.100")
                    .with_port(8765),
            );

        // 生成二维码
        let generator = QrCodeGenerator::new();
        let png = generator.generate_png(&original).unwrap();

        // 解析
        let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();

        // 验证
        assert_eq!(original.device_id, parsed.device_id);
        assert_eq!(original.public_key, parsed.public_key);
        assert_eq!(original.version, parsed.version);

        let orig_info = original.connection_info.as_ref().unwrap();
        let parsed_info = parsed.connection_info.as_ref().unwrap();
        assert_eq!(orig_info.ip, parsed_info.ip);
        assert_eq!(orig_info.port, parsed_info.port);
    }

    #[test]
    fn test_parse_invalid_image_format() {
        let invalid_data = b"not an image";
        let result = QrCodeParser::parse_from_bytes(invalid_data);
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::QrCodeParsing(_))));
    }

    #[test]
    fn test_parse_empty_image() {
        let empty_data: &[u8] = &[];
        let result = QrCodeParser::parse_from_bytes(empty_data);
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::QrCodeParsing(_))));
    }

    #[test]
    fn test_parse_valid_png_without_qrcode() {
        // 创建一个简单的白色 PNG 图片（无二维码）
        use image::{ImageBuffer, Luma};
        let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |_, _| Luma([255u8]));

        let mut png_data = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_data);
        img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();

        let result = QrCodeParser::parse_from_bytes(&png_data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, CryptoError::QrCodeParsing(_)));
        assert!(err.to_string().contains("No QR code found"));
    }

    #[test]
    fn test_parse_pairing_data_invalid_json_in_qrcode() {
        // 这个测试需要一个包含非 JSON 内容的二维码
        // 由于我们的 QrCodeGenerator 总是生成有效的 JSON，
        // 这里我们测试 from_json 的错误处理
        let invalid_json = "not valid json";
        let result = PairingData::from_json(invalid_json);
        assert!(result.is_err());
        assert!(matches!(result, Err(CryptoError::JsonSerialization(_))));
    }

    #[test]
    fn test_parse_pairing_data_invalid_data_in_qrcode() {
        // 测试有效 JSON 但无效配对数据
        let invalid_pairing_json = r#"{"version":1,"device_id":"","public_key":"BAAA"}"#;
        let result = PairingData::from_json(invalid_pairing_json);
        // JSON 解析成功
        assert!(result.is_ok());
        // 但验证失败
        let data = result.unwrap();
        let validation = data.validate();
        assert!(validation.is_err());
    }

    #[test]
    fn test_parse_compressed_public_key() {
        let keypair = EcdhKeyPair::generate();
        let compressed = keypair.public_key_bytes_compressed(); // 33 bytes

        let original = PairingData::new("device".to_string(), &compressed);
        let generator = QrCodeGenerator::new();
        let png = generator.generate_png(&original).unwrap();

        let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();
        assert_eq!(parsed.public_key_bytes().unwrap().len(), 33);
    }

    #[test]
    fn test_parse_with_full_connection_info() {
        let keypair = EcdhKeyPair::generate();
        let original = PairingData::new("full-device".to_string(), &keypair.public_key_bytes())
            .with_connection_info(
                ConnectionInfo::new()
                    .with_ip("10.0.0.1")
                    .with_port(12345)
                    .with_mdns_name("device._nearclip._tcp.local"),
            );

        let generator = QrCodeGenerator::new();
        let png = generator.generate_png(&original).unwrap();
        let parsed = QrCodeParser::parse_pairing_data(&png).unwrap();

        let info = parsed.connection_info.as_ref().unwrap();
        assert_eq!(info.ip, Some("10.0.0.1".to_string()));
        assert_eq!(info.port, Some(12345));
        assert_eq!(info.mdns_name, Some("device._nearclip._tcp.local".to_string()));
    }
}
