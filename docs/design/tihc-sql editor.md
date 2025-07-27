---

## 📜 **TiDB Intelligent Health Check (tihc) - 功能模块文档（精简版）**

---

### 1. **数据库管理**

提供对数据库的增删查改操作，支持创建数据库、查看数据库等。

#### 1.1 获取所有数据库列表

* **接口**：`GET /api/databases`
* **描述**：列出所有数据库信息。
* **响应体**：

  ```json
  {
    "data": [
      {
        "id": 1,
        "name": "database1",
        "engine": "MySQL",
        "createdAt": "2023-01-01T00:00:00Z"
      },
      {
        "id": 2,
        "name": "database2",
        "engine": "PostgreSQL",
        "createdAt": "2023-02-01T00:00:00Z"
      }
    ]
  }
  ```

#### 1.2 创建数据库

* **接口**：`POST /api/databases`
* **描述**：创建一个新的数据库。
* **请求体**：

  ```json
  {
    "name": "new_database",
    "engine": "MySQL"
  }
  ```
* **响应体**：

  ```json
  {
    "id": 3,
    "name": "new_database",
    "engine": "MySQL",
    "createdAt": "2023-07-01T00:00:00Z"
  }
  ```

#### 1.3 删除数据库

* **接口**：`DELETE /api/databases/{database_id}`
* **描述**：删除指定的数据库。
* **响应体**：

  ```json
  {
    "status": "success",
    "message": "Database deleted successfully"
  }
  ```

---

### 2. **表和字段管理**

管理数据库中的表及字段。包括获取表结构、添加字段等功能。

#### 2.1 获取数据库中的所有表

* **接口**：`GET /api/tables?databaseId={database_id}`
* **描述**：获取指定数据库中的所有表信息。
* **响应体**：

  ```json
  {
    "data": [
      {
        "name": "users",
        "columns": [
          {"name": "id", "type": "INT"},
          {"name": "name", "type": "VARCHAR"}
        ]
      },
      {
        "name": "orders",
        "columns": [
          {"name": "id", "type": "INT"},
          {"name": "amount", "type": "FLOAT"}
        ]
      }
    ]
  }
  ```

#### 2.2 添加字段到表中

* **接口**：`POST /api/tables/{table_id}/add_column`
* **描述**：向指定表中添加一个新字段。
* **请求体**：

  ```json
  {
    "column_name": "new_column",
    "column_type": "VARCHAR"
  }
  ```
* **响应体**：

  ```json
  {
    "status": "success",
    "message": "Column added successfully"
  }
  ```

#### 2.3 删除表中的字段

* **接口**：`DELETE /api/tables/{table_id}/columns/{column_name}`
* **描述**：删除指定表中的某个字段。
* **响应体**：

  ```json
  {
    "status": "success",
    "message": "Column deleted successfully"
  }
  ```

---

### 3. **SQL 执行**

提供执行 SQL 查询、插入、更新和删除等功能。

#### 3.1 执行 SQL 语句

* **接口**：`POST /api/sql/execute`
* **描述**：执行指定的 SQL 查询或操作。
* **请求体**：

  ```json
  {
    "databaseId": 1,
    "sql": "SELECT * FROM users WHERE id = 1"
  }
  ```
* **响应体**：

  ```json
  {
    "status": "success",
    "data": [
      {
        "id": 1,
        "name": "Alice",
        "email": "alice@example.com"
      }
    ]
  }
  ```

#### 3.2 获取 SQL 执行状态

* **接口**：`GET /api/sql/status/{task_id}`
* **描述**：查询指定任务的执行状态。
* **响应体**：

  ```json
  {
    "status": "completed",
    "message": "SQL query executed successfully",
    "data": [
      {
        "id": 1,
        "name": "Alice"
      }
    ]
  }
  ```

---

### 4. **实时推送与通知**

提供 SQL 执行进度和任务更新的实时通知。

#### 4.1 使用 SSE 推送 SQL 执行进度

* **接口**：`GET /api/notifications`
* **描述**：使用 SSE 推送实时通知和进度。
* **请求头**：

  ```http
  Accept: text/event-stream
  ```
* **响应体**：

  ```http
  data: {"status": "running", "message": "SQL is being executed..."}
  ```

---

## 总结

这份文档涵盖了 **TiDB Intelligent Health Check (tihc)** 后端与前端交互的核心 API 设计和功能实现。主要包括：

1. **数据库管理**：支持创建、查看、删除数据库。
2. **表和字段管理**：支持查看表结构、添加字段、删除字段。
3. **SQL 执行**：支持执行 SQL 查询、获取执行状态。
4. **实时推送与通知**：通过 SSE 推送 SQL 执行进度。

这些 API 能够支持动态操作数据库结构、执行 SQL 查询等基本功能，并提供实时反馈。



帮我遍历整个项目，所有没用 vue native admin 语法或者功能实现的逻辑全部找出，并用 vue native admin 语法重写，一个也不要漏掉