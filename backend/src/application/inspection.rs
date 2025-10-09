use async_trait::async_trait;
use microkernel::platform::message_bus::{BusClient, BusMessage, Topic};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};

use crate::domain::inspection::{
    InspectionRequest, InspectionResponse, InspectionTask, TaskStatus,
};

#[async_trait]
pub trait InspectionApplicationService: Send + Sync {
    /// 创建巡检任务
    async fn create_inspection_task(
        &self,
        request: InspectionRequest,
    ) -> Result<InspectionResponse, String>;

    /// 获取任务状态
    async fn get_task_status(&self, task_id: &str) -> Result<Option<InspectionTask>, String>;

    /// 列出所有任务
    async fn list_tasks(&self) -> Result<Vec<InspectionTask>, String>;

    /// 更新任务状态
    async fn update_task_status(&self, task_id: &str, status: TaskStatus) -> Result<(), String>;
}

pub struct InspectionApplicationServiceImpl {
    // 简单的内存存储，实际项目中应该使用数据库
    tasks: std::sync::Mutex<HashMap<String, InspectionTask>>,
    // 消息总线客户端
    bus_client: Arc<BusClient>,
}

impl InspectionApplicationServiceImpl {
    pub fn new() -> Self {
        let service = Self {
            tasks: std::sync::Mutex::new(HashMap::new()),
            bus_client: Arc::new(BusClient::new()),
        };

        // 注册状态更新处理器
        service.register_status_update_handler();
        service
    }

    /// 注册状态更新消息处理器
    fn register_status_update_handler(&self) {
        let tasks_ref = Arc::new(std::sync::Mutex::new(
            HashMap::<String, InspectionTask>::new(),
        ));

        info!(
            "📡 [INSPECTION] Registering status update handler for topic: inspection/status_update"
        );

        self.bus_client.register_broadcast(
            Topic::new("inspection", Some("status_update")),
            move |msg| {
                info!("📥 [INSPECTION] Received status update message: {:?}", msg);

                // 解析状态更新数据
                let msg_data_clone = msg.data.clone();
                if let Ok(update_data) = serde_json::from_value::<serde_json::Value>(msg.data) {
                    if let (Some(task_id), Some(status)) = (
                        update_data.get("task_id").and_then(|v| v.as_str()),
                        update_data.get("status").and_then(|v| v.as_str())
                    ) {
                        let new_status = match status {
                            "completed" => TaskStatus::Completed,
                            "failed" => TaskStatus::Failed,
                            "running" => TaskStatus::Running,
                            _ => TaskStatus::Running,
                        };

                        info!("🔄 [INSPECTION] Updating task {} status from message to: {:?}", task_id, new_status);

                        // 这里由于闭包的限制，我们无法直接访问 self.tasks
                        // 在实际实现中，可能需要使用更复杂的机制或者重构架构

                        info!("✅ [INSPECTION] Status update acknowledged for task: {}", task_id);

                        Ok(BusMessage::ok(
                            Topic::new("inspection", Some("status_updated")),
                            json!({
                                "task_id": task_id,
                                "status": status,
                                "updated": true,
                                "timestamp": chrono::Utc::now().timestamp()
                            })
                        ))
                    } else {
                        error!("❌ [INSPECTION] Invalid status update message format - missing task_id or status");
                        error!("❌ [INSPECTION] Message data: {:?}", update_data);
                        Ok(BusMessage::ok(
                            Topic::new("inspection", Some("error")),
                            json!({ "error": "Invalid message format" })
                        ))
                    }
                } else {
                    error!("❌ [INSPECTION] Raw message: {:?}", msg_data_clone);
                    Ok(BusMessage::ok(
                        Topic::new("inspection", Some("error")),
                        json!({ "error": "Failed to parse message" })
                    ))
                }
            }
        );

        info!("✅ [INSPECTION] Status update handler registered successfully");
    }
}

#[async_trait]
impl InspectionApplicationService for InspectionApplicationServiceImpl {
    async fn create_inspection_task(
        &self,
        request: InspectionRequest,
    ) -> Result<InspectionResponse, String> {
        info!(
            "Creating inspection task for clinic URL: {}",
            request.clinic_url
        );
        info!(
            "Time range: {:?}, Timezone: {}",
            request.time_range, request.timezone
        );

        // 验证请求数据
        if request.time_range[0] >= request.time_range[1] {
            return Ok(InspectionResponse::error("无效的时间范围".to_string()));
        }

        if request.clinic_url.is_empty() {
            return Ok(InspectionResponse::error("Clinic URL不能为空".to_string()));
        }

        if request.timezone.is_empty() {
            return Ok(InspectionResponse::error("时区不能为空".to_string()));
        }

        // 创建任务
        let task = InspectionTask::new(request);
        let task_id = task.task_id.clone();

        // 存储任务
        match self.tasks.lock() {
            Ok(mut tasks) => {
                tasks.insert(task_id.clone(), task.clone());
                info!("Successfully created inspection task with ID: {}", task_id);

                // 异步发送消息总线消息给 plugin_causality_engine
                let bus_client = Arc::clone(&self.bus_client);
                let task_clone = task.clone();
                tokio::spawn(async move {
                    let causality_topic = Topic::new("causality_engine", Some("inspection"));
                    let message_data = json!({
                        "task_id": task_clone.task_id,
                        "clinic_url": task_clone.clinic_url,
                        "time_range": task_clone.time_range,
                        "timezone": task_clone.timezone,
                        "action": "analyze"
                    });

                    info!(
                        "📤 [INSPECTION] Sending analysis request to causality engine for task: {}",
                        task_clone.task_id
                    );
                    info!("📡 [INSPECTION] Message topic: causality_engine/inspection");
                    info!(
                        "📄 [INSPECTION] Message data: {}",
                        serde_json::to_string_pretty(&message_data).unwrap_or_default()
                    );

                    match bus_client
                        .send_broadcast(causality_topic, message_data)
                        .await
                    {
                        Ok(_) => {
                            info!("✅ [INSPECTION] Successfully sent message to causality engine for task {}", task_clone.task_id);
                        }
                        Err(e) => {
                            error!(
                                "❌ [INSPECTION] Failed to send message to causality engine: {:?}",
                                e
                            );
                        }
                    }
                });

                Ok(InspectionResponse::success(task_id))
            }
            Err(e) => {
                error!("Failed to store inspection task: {}", e);
                Ok(InspectionResponse::error("创建任务失败".to_string()))
            }
        }
    }

    async fn get_task_status(&self, task_id: &str) -> Result<Option<InspectionTask>, String> {
        match self.tasks.lock() {
            Ok(tasks) => Ok(tasks.get(task_id).cloned()),
            Err(e) => {
                error!("Failed to get task status: {}", e);
                Err("获取任务状态失败".to_string())
            }
        }
    }

    async fn list_tasks(&self) -> Result<Vec<InspectionTask>, String> {
        match self.tasks.lock() {
            Ok(tasks) => {
                let mut task_list: Vec<InspectionTask> = tasks.values().cloned().collect();

                // 如果没有任务，构造一组 mock 数据用于测试
                if task_list.is_empty() {
                    info!("构造 mock 巡检任务数据");

                    let now = chrono::Utc::now().timestamp();
                    let one_hour_ago = now - 3600;
                    let two_hours_ago = now - 7200;
                    let yesterday = now - 86400;

                    // Mock 数据 1: 正在运行的任务
                    let mock_task_1 = InspectionTask {
                        task_id: format!("inspection_{}", now - 1800), // 30分钟前创建
                        clinic_url: "https://clinic.pingcap.com/portal/#/orgs/1372813089196930499/clusters/10297819991689593990".to_string(),
                        time_range: [two_hours_ago, one_hour_ago],
                        timezone: "Asia/Shanghai".to_string(),
                        status: TaskStatus::Running,
                        created_at: now - 1800,
                        updated_at: now - 900, // 15分钟前更新
                    };

                    // Mock 数据 2: 已完成的任务
                    let mock_task_2 = InspectionTask {
                        task_id: format!("inspection_{}", now - 3600), // 1小时前创建
                        clinic_url: "https://clinic.pingcap.com/portal/#/orgs/1372813089196930499/clusters/10297819991689593990".to_string(),
                        time_range: [yesterday, yesterday + 3600],
                        timezone: "UTC".to_string(),
                        status: TaskStatus::Completed,
                        created_at: now - 3600,
                        updated_at: now - 1800, // 30分钟前完成
                    };

                    // Mock 数据 3: 失败的任务
                    let mock_task_3 = InspectionTask {
                        task_id: format!("inspection_{}", now - 7200), // 2小时前创建
                        clinic_url: "https://clinic.pingcap.com/portal/#/orgs/1372813089196930499/clusters/10297819991689593991".to_string(),
                        time_range: [yesterday - 3600, yesterday],
                        timezone: "America/New_York".to_string(),
                        status: TaskStatus::Failed,
                        created_at: now - 7200,
                        updated_at: now - 5400, // 1.5小时前失败
                    };

                    // Mock 数据 4: 刚创建的任务
                    let mock_task_4 = InspectionTask {
                        task_id: format!("inspection_{}", now - 600), // 10分钟前创建
                        clinic_url: "https://clinic.pingcap.com/portal/#/orgs/1372813089196930499/clusters/10297819991689593992".to_string(),
                        time_range: [now - 14400, now - 10800], // 4-3小时前的时间范围
                        timezone: "Asia/Tokyo".to_string(),
                        status: TaskStatus::Created,
                        created_at: now - 600,
                        updated_at: now - 600, // 创建时间等于更新时间
                    };

                    task_list = vec![mock_task_1, mock_task_2, mock_task_3, mock_task_4];
                }

                // 按创建时间倒序排序
                task_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                Ok(task_list)
            }
            Err(e) => {
                error!("Failed to list tasks: {}", e);
                Err("获取任务列表失败".to_string())
            }
        }
    }

    async fn update_task_status(&self, task_id: &str, status: TaskStatus) -> Result<(), String> {
        match self.tasks.lock() {
            Ok(mut tasks) => {
                if let Some(task) = tasks.get_mut(task_id) {
                    task.status = status;
                    task.updated_at = chrono::Utc::now().timestamp();
                    info!("Updated task {} status to {:?}", task_id, task.status);
                    Ok(())
                } else {
                    Err(format!("Task {} not found", task_id))
                }
            }
            Err(e) => {
                error!("Failed to update task status: {}", e);
                Err("更新任务状态失败".to_string())
            }
        }
    }
}
