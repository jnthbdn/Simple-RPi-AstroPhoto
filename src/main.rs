use actix_web::{web, App, HttpServer};
use actix_files as fs;
use std::sync::Mutex;

pub mod rpi_cam;
pub mod routes;

use rpi_cam::RpiCam;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = web::Data::new( Mutex::new(RpiCam::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(routes::take_photo)

            .service(routes::set_width)
            .service(routes::set_height)
            .service(routes::set_hflip)
            .service(routes::set_vflip)
            .service(routes::set_rotation)
            .service(routes::set_shutter_speed)
            .service(routes::set_sharpness)
            .service(routes::set_contrast)
            .service(routes::set_brightness)
            .service(routes::set_saturation)
            .service(routes::set_iso)
            .service(routes::set_stabilization)
            .service(routes::set_ev_compensation)
            .service(routes::set_exposure)
            .service(routes::set_analog_gain)
            .service(routes::set_digital_gain)
            .service(routes::set_awb)
            .service(routes::set_awb_blue)
            .service(routes::set_awb_red)
            .service(routes::set_effect)
            .service(routes::set_metering)
            .service(routes::set_drc)
            .service(fs::Files::new("/", "static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}