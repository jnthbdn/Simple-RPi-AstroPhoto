use actix_web::{web, App, HttpServer};
use actix_files as fs;
use std::sync::{Arc, Mutex};

pub mod rpi_cam;
pub mod routes;
mod constants;

use rpi_cam::RpiCam;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    
    println!("Start server on address : 0.0.0.0:8080");

    let arc_data = Arc::new(Mutex::new(RpiCam::new()));
    let local_data = arc_data.clone();

    let server = HttpServer::new(move || {
        let server_data = web::Data::new(arc_data.clone());

        let mut mutex = server_data.lock().unwrap();
        mutex.start_preview();
        drop(mutex);

        App::new()
            .app_data(server_data)
            .service(routes::take_photo)
            .service(routes::preview)

            .service(routes::set_width)
            .service(routes::set_height)
            .service(routes::set_hflip)
            .service(routes::set_vflip)
            .service(routes::set_rotation)
            .service(routes::set_quality)
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
    });
    
    let result = server.bind("0.0.0.0:8080")?
          .run()
          .await;

    local_data.lock().unwrap().stop_preview();

    return result;
}