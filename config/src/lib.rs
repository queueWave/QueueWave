use config::{Config, File};
use std::sync::{Arc, Once};

static INIT: Once = Once::new();
static mut CONFIG: Option<Arc<Config>> = None;



pub fn get_config() -> Arc<Config> {
    unsafe {
        INIT.call_once(|| {
            let mut settings = Config::default();
            settings.merge(File::with_name("application.yml")).unwrap();
            CONFIG = Some(Arc::new(settings));
        });
        CONFIG.clone().unwrap()
    }
}

pub fn get_value(key: &str) -> Option<String> {
    let config = get_config();
    config.get::<String>(key).ok()
}