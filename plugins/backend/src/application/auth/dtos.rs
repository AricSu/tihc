// 应用层 DTOs：定义应用服务的输入输出格式
// 符合 DDD 原则：应用层负责协调和数据转换

use crate::domain::auth::UserInfo;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 登录请求 DTO
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 50))]
    pub email: String,
    #[validate(length(min = 1, max = 50))]
    pub password: String,
    #[validate(length(min = 4, max = 4))]
    pub captcha: String,
    pub captcha_session_id: String,
}

/// 前端期望的用户详情响应格式
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetailResponse {
    pub id: i64,
    pub username: String,
    pub profile: UserProfile,
    pub roles: Vec<String>, // 暂时为空，后续可扩展
    #[serde(rename = "currentRole")]
    pub current_role: Option<String>, // 暂时为空，后续可扩展
}

/// 用户配置文件信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub avatar: Option<String>,
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
    pub email: String,
    pub gender: Option<String>,  // 暂时为空，后续可扩展
    pub address: Option<String>, // 暂时为空，后续可扩展
}

impl From<UserInfo> for UserDetailResponse {
    fn from(user: UserInfo) -> Self {
        Self {
            id: user.id,
            username: user.username,
            profile: UserProfile {
                avatar: user.avatar,
                nick_name: user.nick_name,
                email: user.email,
                gender: None,  // 未来可从用户扩展信息中获取
                address: None, // 未来可从用户扩展信息中获取
            },
            roles: Vec::new(),  // 未来从权限系统中获取
            current_role: None, // 未来从权限系统中获取
        }
    }
}

/// 错误响应 DTO
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
}

/// 用户列表请求 DTO
#[derive(Debug, Deserialize, Validate)]
pub struct UserListRequest {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    pub keyword: Option<String>,
    pub status: Option<String>,
}

fn default_page() -> i32 {
    1
}
fn default_page_size() -> i32 {
    20
}

/// 用户列表响应 DTO
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub list: Vec<UserListItem>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

/// 用户列表项 DTO
#[derive(Debug, Serialize)]
pub struct UserListItem {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建用户请求 DTO
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 100))]
    pub password: String,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
}

/// 更新用户请求 DTO
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
}

/// 修改密码请求 DTO
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 6, max = 100))]
    pub old_password: String,
    #[validate(length(min = 6, max = 100))]
    pub new_password: String,
}

/// 权限树节点 DTO
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionNode {
    pub id: i64,
    pub name: String,
    pub key: String,
    #[serde(default)]
    pub children: Vec<PermissionNode>,
}
