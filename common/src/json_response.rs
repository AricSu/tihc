#[macro_export]
macro_rules! json_resp {
    // NotFound + data (用于 Some/None 分支类型一致)
    (NotFound, $data:expr) => {
        common::json_response::JsonResponse::new(
            common::json_response::RespStatus::NotFound,
            None,
            None,
            $data,
            None,
        )
    };
    (success, $data:expr) => {
        common::json_response::JsonResponse::success($data)
    };
    ($status:ident) => {
        common::json_response::JsonResponse::<()>::status(
            common::json_response::RespStatus::$status,
        )
    };
    ($status:ident, $msg:expr) => {
        common::json_response::JsonResponse::<()>::status_msg(
            common::json_response::RespStatus::$status,
            $msg,
        )
    };
    ($status:ident, $code:expr, $msg:expr) => {
        common::json_response::JsonResponse::<()>::new(
            common::json_response::RespStatus::$status,
            Some($code.to_string()),
            Some($msg.into()),
            None,
            None,
        )
    };
    ($status:ident, $code:expr, $msg:expr, $data:expr) => {
        common::json_response::JsonResponse::new(
            common::json_response::RespStatus::$status,
            Some($code.to_string()),
            Some($msg.into()),
            Some($data),
            None,
        )
    };
    ($status:ident, $code:expr, $msg:expr, $data:expr, $trace_id:expr) => {
        common::json_response::JsonResponse::new(
            common::json_response::RespStatus::$status,
            Some($code.to_string()),
            Some($msg.into()),
            Some($data),
            Some($trace_id.to_string()),
        )
    };
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RespStatus {
    Success,
    Failed,
    NotFound,
    UnsupportedEngine,
    // 可扩展更多业务状态
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum RespMessage {
    Text(String),
    I18n {
        code: String,
        args: Option<serde_json::Value>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonResponse<T> {
    pub status: RespStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<RespMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl<T> JsonResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: RespStatus::Success,
            code: None,
            message: None,
            data: Some(data),
            trace_id: None,
        }
    }

    pub fn status(status: RespStatus) -> Self {
        Self {
            status,
            code: None,
            message: None,
            data: None,
            trace_id: None,
        }
    }

    pub fn status_msg(status: RespStatus, msg: impl Into<RespMessage>) -> Self {
        Self {
            status,
            code: None,
            message: Some(msg.into()),
            data: None,
            trace_id: None,
        }
    }
    pub fn new(
        status: RespStatus,
        code: Option<String>,
        message: Option<RespMessage>,
        data: Option<T>,
        trace_id: Option<String>,
    ) -> Self {
        Self {
            status,
            code,
            message,
            data,
            trace_id,
        }
    }
}

impl From<String> for RespMessage {
    fn from(s: String) -> Self {
        RespMessage::Text(s)
    }
}
impl<'a> From<&'a str> for RespMessage {
    fn from(s: &'a str) -> Self {
        RespMessage::Text(s.to_string())
    }
}
