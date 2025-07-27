pub struct ServiceRegistry {
    services: std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}


impl ServiceRegistry {
    pub fn new() -> Self {
        ServiceRegistry {
            services: std::collections::HashMap::new(),
        }
    }

    pub fn register<T: Send + Sync + 'static>(&mut self, service: Box<T>) {
        self.services.insert(std::any::type_name::<T>().to_string(), service);
    }

    pub fn resolve<T: 'static>(&self) -> Option<&T> {
        self.services
            .get(&std::any::type_name::<T>().to_string())
            .and_then(|s| s.downcast_ref::<T>())
    }
}
