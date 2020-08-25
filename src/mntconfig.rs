#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Light {
    pub name: String,
    pub title: String
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct MqttServer {
    pub host: String,
    pub port: u16
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub bind: String,
    pub mqtt: MqttServer,
    pub lights: Vec<Light>
}
