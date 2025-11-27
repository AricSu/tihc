use captcha::{
    Captcha,
    filters::{Noise, Wave},
};

use crate::infrastructure::CaptchaRepository;

/// 验证码信息值对象
#[derive(Debug, Clone)]
pub struct CaptchaInfo {
    pub text: String,
    pub image_data: Vec<u8>,
    pub session_id: String,
}

pub struct CaptchaService<R: CaptchaRepository> {
    repository: R,
    expiry_seconds: u64,
}

impl<R: CaptchaRepository> CaptchaService<R> {
    pub fn new(repository: R, expiry_seconds: u64) -> Self {
        Self {
            repository,
            expiry_seconds,
        }
    }

    pub fn generate_captcha(
        &self,
    ) -> Result<CaptchaInfo, Box<dyn std::error::Error + Send + Sync>> {
        // Create a captcha with custom configuration
        let mut captcha = Captcha::new();

        // Set characters to use (avoid ambiguous ones like 0, O, l, I)
        let chars = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S',
            'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7', '8', '9',
        ];

        captcha
            .set_chars(&chars)
            .add_chars(4) // 4 characters
            .apply_filter(Noise::new(0.1)) // Light noise
            .apply_filter(Wave::new(2.0, 20.0)) // Wave distortion
            .view(160, 60); // Set image size

        // Get the text that was generated
        let text = captcha.chars_as_string();

        // Generate the image as PNG bytes
        let image_data = captcha.as_png().ok_or("Failed to generate captcha PNG")?;

        // Generate session ID
        let session_id = self.generate_session_id();

        // Store in repository
        self.repository
            .save(&session_id, &text.to_uppercase(), self.expiry_seconds);

        Ok(CaptchaInfo {
            text: text.to_uppercase(),
            image_data,
            session_id,
        })
    }

    pub fn validate_captcha(&self, session_id: &str, input: &str) -> bool {
        self.repository.validate(session_id, input)
    }

    pub fn remove_captcha(&self, session_id: &str) {
        self.repository.remove(session_id);
    }

    /// Generate a cryptographically secure session ID
    fn generate_session_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}
