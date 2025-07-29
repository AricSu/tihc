pub struct ServiceRegistry {
    services: std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        ServiceRegistry {
            services: std::collections::HashMap::new(),
        }
    }

    pub fn register<T: Sized + Send + Sync + 'static>(&mut self, service: T) {
        self.services.insert(
            std::any::type_name::<T>().to_string(),
            Box::new(service) as Box<dyn std::any::Any + Send + Sync>,
        );
    }

    pub fn resolve<T: 'static>(&self) -> Option<&T> {
        self.services
            .get(&std::any::type_name::<T>().to_string())
            .and_then(|s| s.downcast_ref::<T>())
    }
}
