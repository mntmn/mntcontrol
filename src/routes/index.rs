use actix_web::{error, get, put, web, Error, HttpResponse};
use serde_json::json;
use rumqtt::{MqttClient, MqttOptions, QoS};
use crate::mntconfig::Config;

#[get("/")]
pub async fn get_index(
    tmpl: web::Data<tera::Tera>,
    config: web::Data<Config>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    ctx.insert("lights", &config.lights);

    let s = tmpl.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LightControl {
    brightness: i32
}

#[put("/lights/{id}")]
pub async fn put_lights_json(
    params: web::Json<LightControl>,
    path: web::Path<(String,)>,
    config: web::Data<Config>
) -> Result<HttpResponse, Error> {
    let mqtt_options = MqttOptions::new("mntcontrol", &config.mqtt.host, config.mqtt.port);
    match MqttClient::start(mqtt_options) {
        Ok((mut mqtt_client, _notifications)) => {
            let light:&LightControl = &params;

            let topic = format!("zigbee2mqtt/{}/set",&path.0);
            let msg = serde_json::to_string(light).unwrap();

            match mqtt_client.publish(topic, QoS::AtLeastOnce, false, msg) {
                Err(e) => {
                    Err(error::ErrorInternalServerError(json!({
                        "success": false,
                        "error":format!("mqtt publish error: {:?}", e)
                    })))
                }
                _ => {
                    Ok(HttpResponse::Ok().content_type("application/json").body(json!({
                        "success": true
                    })))
                }
            }
        },
        Err(e) => {
            Err(error::ErrorInternalServerError(json!({
                "success": false,
                "error":format!("mqtt connection error: {:?}", e)
            })))
        }
    }
}
