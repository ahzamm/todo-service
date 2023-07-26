use redis::{Client, Commands, RedisResult};

pub fn create_redis_client() -> Client {
    redis::Client::open("redis://127.0.0.1:6379/").expect("Failed to connect to Redis")
}

pub fn get_data_from_redis(key: &str) -> RedisResult<Option<String>> {
    let mut connection = create_redis_client().get_connection()?;
    connection.get(key)
}

pub fn set_data_in_redis(key: &str, data: &str) -> RedisResult<()> {
    let mut connection = create_redis_client().get_connection()?;
    connection.set(key, data)?;
    Ok(())
}
