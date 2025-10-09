use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{error, info};

use crate::application::inspection::InspectionApplicationService;
use crate::domain::inspection::{InspectionRequest, InspectionResponse, TaskStatus};

#[derive(Clone)]
pub struct InspectionAppState {
    service: Arc<dyn InspectionApplicationService>,
}

impl InspectionAppState {
    pub fn new(service: Arc<dyn InspectionApplicationService>) -> Self {
        Self { service }
    }
}

pub struct InspectionController;

impl InspectionController {
    pub fn routes() -> Router<InspectionAppState> {
        Router::new()
            .route("/api/inspection/create", post(Self::create_inspection_task))
            .route("/api/inspection/summary", get(Self::get_inspection_summary))
            .route(
                "/api/inspection/tasks/{task_id}",
                get(Self::get_task_detail),
            )
    }

    pub async fn create_inspection_task(
        State(state): State<InspectionAppState>,
        Json(request): Json<InspectionRequest>,
    ) -> Result<Json<InspectionResponse>, (StatusCode, Json<Value>)> {
        info!(
            "收到巡检任务创建请求 - Clinic URL: {}, 时间范围: {:?}",
            request.clinic_url, request.time_range
        );

        match state.service.create_inspection_task(request).await {
            Ok(response) => {
                info!("巡检任务创建成功 - Task ID: {:?}", response.task_id);
                Ok(Json(response))
            }
            Err(error) => {
                error!("巡检任务创建异常: {}", error);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "message": error,
                        "task_id": null
                    })),
                ))
            }
        }
    }

    pub async fn get_inspection_summary(
        State(state): State<InspectionAppState>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        info!("获取巡检摘要");

        match state.service.list_tasks().await {
            Ok(tasks) => {
                let summary_items: Vec<Value> = tasks
                    .into_iter()
                    .map(|task| match task.status {
                        TaskStatus::Created => {
                            json!({
                                "id": task.task_id,
                                "type": "created_task",
                                "title": format!("待执行巡检 - {}", &task.task_id[11..19]),
                                "status": "created",
                                "timeRange": task.time_range,
                                "clinicUrl": task.clinic_url,
                                "timezone": task.timezone,
                                "createTime": task.created_at
                            })
                        }
                        TaskStatus::Running => {
                            json!({
                                "id": task.task_id,
                                "type": "running_task",
                                "title": format!("正在执行巡检 - {}", &task.task_id[11..19]),
                                "status": "running",
                                "timeRange": task.time_range,
                                "clinicUrl": task.clinic_url,
                                "timezone": task.timezone,
                                "createTime": task.created_at
                            })
                        }
                        TaskStatus::Completed => {
                            json!({
                                "id": task.task_id,
                                "type": "inspection_report",
                                "title": format!("巡检报告 - {}", &task.task_id[11..19]),
                                "status": "completed",
                                "timeRange": task.time_range,
                                "clinicUrl": task.clinic_url,
                                "timezone": task.timezone,
                                "createTime": task.created_at,
                                "healthStatus": "Healthy"
                            })
                        }
                        TaskStatus::Failed => {
                            json!({
                                "id": task.task_id,
                                "type": "failed_task",
                                "title": format!("巡检失败 - {}", &task.task_id[11..19]),
                                "status": "failed",
                                "timeRange": task.time_range,
                                "clinicUrl": task.clinic_url,
                                "timezone": task.timezone,
                                "createTime": task.created_at
                            })
                        }
                    })
                    .collect();

                Ok(Json(json!({
                    "success": true,
                    "summary": {
                        "total": summary_items.len(),
                        "items": summary_items
                    }
                })))
            }
            Err(error) => {
                error!("获取巡检摘要失败: {}", error);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "message": error
                    })),
                ))
            }
        }
    }

    pub async fn get_task_detail(
        State(state): State<InspectionAppState>,
        Path(task_id): Path<String>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        info!("查询任务详情 - Task ID: {}", task_id);

        match state.service.get_task_status(&task_id).await {
            Ok(Some(task)) => Ok(Json(json!({
                "success": true,
                "task": task
            }))),
            Ok(None) => Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "message": "任务不存在"
                })),
            )),
            Err(error) => {
                error!("查询任务详情失败: {}", error);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "message": error
                    })),
                ))
            }
        }
    }
}
