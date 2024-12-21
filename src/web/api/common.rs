use axum::Extension;
use tower_sessions::Session;

use crate::web::session::{ApiKey, Cache, API_KEY};

pub async fn get_user_id(cache: Extension<Cache>, session: Session) -> Result<Option<i32>, ()> {
    if let Ok(Some(key)) = session.get::<ApiKey>(API_KEY).await {
        if let Ok(Some(user_id)) = cache.read().map(|hm| hm.get(&key.0).map(|v| v.clone())) {
            i32::from_str_radix(&user_id, 10)
                .map_err(|_| ())
                .map(|v| Some(v))
        } else {
            Ok(None)
        }
    } else {
        Err(())
    }
}
