//! 自动批量生成插件服务注册和 handler 解耦代码的脚本示例
//! 用法：可在 build.rs 或手动运行，扫描 application/ 目录下所有 *Service trait 和实现，生成注册/handler 模板。

use std::fs;
use std::path::Path;

fn main() {
    let app_dir = "plugins/plugin_slowlog/src/application";
    let out_plugin = "plugins/plugin_slowlog/src/plugin_service_gen.rs";
    let mut trait_impls = Vec::new();
    let mut handler_templates = Vec::new();

    for entry in fs::read_dir(app_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map(|s| s == "rs").unwrap_or(false) {
            let content = fs::read_to_string(&path).unwrap();
            for line in content.lines() {
                if let Some(trait_name) = line.strip_prefix("pub trait ") {
                    let trait_name = trait_name.split(':').next().unwrap().trim();
                    let impl_name = format!("{}Impl", trait_name);
                    // 生成注册代码
                    trait_impls.push(format!(
                        "ctx.service_registry.register::<Box<dyn {0}>>(Box::new(Box::new({1}::default())));",
                        trait_name, impl_name
                    ));
                    // 生成 handler 模板
                    handler_templates.push(format!(
                        "pub async fn handle_{0}_api(Extension(registry): Extension<Arc<dyn IServiceRegistry>>, Json(payload): Json<Value>) -> Json<Value> {{\n    if let Some(service) = registry.resolve::<Box<dyn {0}>>() {{\n        // 调用 trait 方法\n    }} else {{\n        Json(serde_json::json!({{ \"error\": \"service not found\" }}))\n    }}\n}}",
                        trait_name.to_lowercase()
                    ));
                }
            }
        }
    }
    // 输出到文件
    let mut out = String::new();
    out.push_str("// 自动生成的插件服务注册代码\n");
    for reg in &trait_impls {
        out.push_str(reg);
        out.push('\n');
    }
    out.push_str("\n// 自动生成的 handler 模板\n");
    for h in &handler_templates {
        out.push_str(h);
        out.push_str("\n\n");
    }
    fs::write(out_plugin, out).unwrap();
}
