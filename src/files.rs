// File Service - Clean Architecture File Management
// File upload/download, compression, deduplication, and QR generation

use crate::domain::CreateFile;
use crate::error::{AppError, Result};
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use image::GenericImageView;
use qrcode::QrCode;

// ============================================================================
// FILE SERVICE
// ============================================================================

pub struct FileService {
    upload_dir: PathBuf,
}

impl FileService {
    pub fn new(upload_dir: PathBuf) -> Self {
        Self { upload_dir }
    }

    pub async fn ensure_upload_dir(&self) -> Result<()> {
        if !self.upload_dir.exists() {
            fs::create_dir_all(&self.upload_dir).await?;
        }
        Ok(())
    }

    pub async fn calculate_sha256(file_path: &Path) -> Result<String> {
        let mut file = fs::File::open(file_path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    pub async fn upload_file(
        &self,
        institution_id: i64,
        original_name: String,
        file_data: Vec<u8>,
        mime_type: Option<String>,
        uploaded_by: Option<i64>,
    ) -> Result<CreateFile> {
        self.ensure_upload_dir().await?;

        // Calculate SHA256 hash for deduplication
        let mut hasher = Sha256::new();
        hasher.update(&file_data);
        let hash = hasher.finalize();
        let stored_name = format!("{:x}", hash);

        // Check if file already exists (deduplication)
        let file_path = self.upload_dir.join(&stored_name);
        
        if !file_path.exists() {
            // Save file
            let mut file = fs::File::create(&file_path).await?;
            file.write_all(&file_data).await?;
            file.flush().await?;
        }

        Ok(CreateFile {
            institution_id,
            original_name,
            stored_name,
            mime_type,
            size_bytes: Some(file_data.len() as i64),
            uploaded_by,
        })
    }

    pub async fn get_file(&self, stored_name: &str) -> Result<Vec<u8>> {
        let file_path = self.upload_dir.join(stored_name);
        
        if !file_path.exists() {
            return Err(AppError::not_found(format!("File not found: {}", stored_name)));
        }

        let mut file = fs::File::open(&file_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        Ok(buffer)
    }

    pub async fn delete_file(&self, stored_name: &str) -> Result<()> {
        let file_path = self.upload_dir.join(stored_name);
        
        if file_path.exists() {
            fs::remove_file(&file_path).await?;
        }

        Ok(())
    }

    pub async fn get_file_path(&self, stored_name: &str) -> Result<PathBuf> {
        let file_path = self.upload_dir.join(stored_name);
        
        if !file_path.exists() {
            return Err(AppError::not_found(format!("File not found: {}", stored_name)));
        }

        Ok(file_path)
    }
}

// ============================================================================
// IMAGE COMPRESSION SERVICE
// ============================================================================

pub struct ImageCompressionService;

impl ImageCompressionService {
    pub fn compress_image(image_data: &[u8], max_width: u32) -> Result<Vec<u8>> {
        // Load image
        let img = image::load_from_memory(image_data)
            .map_err(|e| AppError::image_processing(format!("Failed to load image: {}", e)))?;

        // Resize image
        let (original_width, original_height) = img.dimensions();
        
        let new_width = if original_width > max_width {
            max_width
        } else {
            original_width
        };

        let ratio = new_width as f32 / original_width as f32;
        let new_height = (original_height as f32 * ratio) as u32;

        let resized = image::imageops::resize(
            &img,
            new_width,
            new_height,
            image::imageops::FilterType::Lanczos3,
        );

        // Encode to JPEG
        let mut buffer = Vec::new();
        resized.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Jpeg)
            .map_err(|e| AppError::image_processing(format!("Failed to encode image: {}", e)))?;

        Ok(buffer)
    }

    pub fn is_image(mime_type: &str) -> bool {
        mime_type.starts_with("image/")
    }

    pub fn get_mime_type(file_data: &[u8]) -> Option<String> {
        // Simple MIME type detection based on magic bytes
        if file_data.len() < 4 {
            return None;
        }

        let magic = &file_data[..4];
        
        match magic {
            [0x89, 0x50, 0x4E, 0x47] => Some("image/png".to_string()),
            [0xFF, 0xD8, 0xFF, _] => Some("image/jpeg".to_string()),
            [0x47, 0x49, 0x46, 0x38] => Some("image/gif".to_string()),
            [0x42, 0x4D, _, _] => Some("image/bmp".to_string()),
            [0x25, 0x50, 0x44, 0x46] => Some("application/pdf".to_string()),
            _ => None,
        }
    }
}

// ============================================================================
// QR CODE GENERATION SERVICE
// ============================================================================

pub struct QrCodeService;

impl QrCodeService {
    pub fn generate_qr_code(data: &str) -> Result<Vec<u8>> {
        use qrcode::render::svg;

        let qr_code = QrCode::new(data)
            .map_err(|e| AppError::image_processing(format!("Failed to generate QR code: {}", e)))?;

        let image = qr_code.render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#FFFFFF"))
            .build();

        Ok(image.into_bytes())
    }

    pub fn generate_certificate_qr(student_id: i64, institution_id: i64, cert_type: &str) -> String {
        // Generate unique QR code data
        use md5;
        let data = format!("{}:{}:{}:{}", institution_id, student_id, cert_type, chrono::Utc::now().timestamp());
        let hash = format!("{:x}", md5::compute(data));
        hash
    }

    pub fn verify_certificate_qr(qr_code: &str) -> Result<bool> {
        // Verify QR code format (basic validation)
        if qr_code.len() != 32 {
            return Err(AppError::validation("Invalid QR code format"));
        }

        // Check if it's a valid hex string
        qr_code.chars().all(|c| c.is_ascii_hexdigit())
            .then_some(true)
            .ok_or_else(|| AppError::validation("Invalid QR code format"))
    }
}

// ============================================================================
// FILE VALIDATION
// ============================================================================

pub struct FileValidator;

impl FileValidator {
    pub const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB
    pub const ALLOWED_MIME_TYPES: &[&str] = &[
        "image/jpeg",
        "image/png",
        "image/gif",
        "image/webp",
        "application/pdf",
        "text/plain",
        "application/msword",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "application/vnd.ms-excel",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    ];

    pub fn validate_file_size(size: usize) -> Result<()> {
        if size > Self::MAX_FILE_SIZE {
            return Err(AppError::validation(format!(
                "File size exceeds maximum allowed size of {}MB",
                Self::MAX_FILE_SIZE / (1024 * 1024)
            )));
        }
        Ok(())
    }

    pub fn validate_mime_type(mime_type: &str) -> Result<()> {
        if !Self::ALLOWED_MIME_TYPES.contains(&mime_type) {
            return Err(AppError::validation(format!(
                "File type '{}' is not allowed",
                mime_type
            )));
        }
        Ok(())
    }

    pub fn validate_file_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(AppError::validation("File name cannot be empty"));
        }

        if name.len() > 255 {
            return Err(AppError::validation("File name is too long (max 255 characters)"));
        }

        // Check for invalid characters
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(AppError::validation("File name contains invalid characters"));
        }

        Ok(())
    }
}
