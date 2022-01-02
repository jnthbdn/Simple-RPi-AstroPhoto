use chrono::prelude::Local;
use actix_web::{get, post, web, HttpResponse};
use std::sync::{Arc, Mutex};
use std::fs;

use crate::rpi_cam::RpiCam;
use crate::constants::*;

type MutexRpiCam = Arc<Mutex<RpiCam>>;

#[get("/preview")]
pub async fn preview(data: web::Data<MutexRpiCam>) -> HttpResponse {
    data.lock().unwrap().check_preview_status();

    let pic = fs::read(FILENAME_PREVIEW);
    match pic{
        Ok(_) => HttpResponse::Ok().content_type("image/jpg").body(pic.unwrap()),
        Err(e) => {
            eprintln!("PREVIEW ERROR : {}", e);

            let pic = fs::read("static/img/default-preview.jpg");
            match pic {
                Ok(_) => HttpResponse::Ok().content_type("image/jpg").body(pic.unwrap()),
                Err(e) =>{
                    eprintln!("DEFAULT PREVIEW ERROR : {}", e);
                    return HttpResponse::InternalServerError().body(format!("{}", e));
                }
            }
        }
    }
}

#[get("/take_photo")]
pub async fn take_photo(data: web::Data<MutexRpiCam>) -> HttpResponse {

    let filename = format!("photo_{}.jpg", Local::now().format("%Y-%m-%d_%H:%M:%S"));

    let pic = data.lock().unwrap().take_pic(&capture_path(&filename));

    match pic{
        Ok(_) => HttpResponse::Ok().body("Done !"),
        Err(s) => HttpResponse::InternalServerError().body(s)
    }
}

#[post("/width/{value}")]
pub async fn set_width(data: web::Data<MutexRpiCam>, path: web::Path<u32>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).width = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/height/{value}")]
pub async fn set_height(data: web::Data<MutexRpiCam>, path: web::Path<u32>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).height = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/hflip/{value}")]
pub async fn set_hflip(data: web::Data<MutexRpiCam>, path: web::Path<bool>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).hflip = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/vflip/{value}")]
pub async fn set_vflip(data: web::Data<MutexRpiCam>, path: web::Path<bool>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).vflip = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/rotation/{value}")]
pub async fn set_rotation(data: web::Data<MutexRpiCam>, path: web::Path<u32>) -> HttpResponse {
    

    if *path != 0 && *path != 90 && *path != 180 && *path != 270 {
        return HttpResponse::BadRequest().body("The Rotation value can only be : 0, 90, 180 or 270.");
    }

    let mut rpi = data.lock().unwrap();
    (*rpi).rotation = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/shutterspeed/{value}")]
pub async fn set_shutter_speed(data: web::Data<MutexRpiCam>, path: web::Path<u32>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).shutter_speed = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/sharpness/{value}")]
pub async fn set_sharpness(data: web::Data<MutexRpiCam>, path: web::Path<i8>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).sharpness = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/contrast/{value}")]
pub async fn set_contrast(data: web::Data<MutexRpiCam>, path: web::Path<i8>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).contrast = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/brightness/{value}")]
pub async fn set_brightness(data: web::Data<MutexRpiCam>, path: web::Path<i8>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).brightness = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/saturation/{value}")]
pub async fn set_saturation(data: web::Data<MutexRpiCam>, path: web::Path<i8>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).saturation = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/iso/{value}")]
pub async fn set_iso(data: web::Data<MutexRpiCam>, path: web::Path<u16>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).iso = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/stabilization/{value}")]
pub async fn set_stabilization(data: web::Data<MutexRpiCam>, path: web::Path<bool>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).stabilization = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/evcompensation/{value}")]
pub async fn set_ev_compensation(data: web::Data<MutexRpiCam>, path: web::Path<i8>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).ev_compensation = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/exposure/{value}")]
pub async fn set_exposure(data: web::Data<MutexRpiCam>, path: web::Path<String>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).exposure = path.to_string();

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/analoggain/{value}")]
pub async fn set_analog_gain(data: web::Data<MutexRpiCam>, path: web::Path<f32>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).analog_gain = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/digitalgain/{value}")]
pub async fn set_digital_gain(data: web::Data<MutexRpiCam>, path: web::Path<f32>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).digital_gain = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/awb/{value}")]
pub async fn set_awb(data: web::Data<MutexRpiCam>, path: web::Path<String>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).awb = path.to_string();

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/awbblue/{value}")]
pub async fn set_awb_blue(data: web::Data<MutexRpiCam>, path: web::Path<f32>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).awb_blue = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/awbred/{value}")]
pub async fn set_awb_red(data: web::Data<MutexRpiCam>, path: web::Path<f32>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).awb_red = *path;

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/effect/{value}")]
pub async fn set_effect(data: web::Data<MutexRpiCam>, path: web::Path<String>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).effect = path.to_string();

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/metering/{value}")]
pub async fn set_metering(data: web::Data<MutexRpiCam>, path: web::Path<String>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).metering = path.to_string();

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}

#[post("/drc/{value}")]
pub async fn set_drc(data: web::Data<MutexRpiCam>, path: web::Path<String>) -> HttpResponse {
    
    let mut rpi = data.lock().unwrap();
    (*rpi).drc = path.to_string();

    rpi.restart_preview();

    HttpResponse::Ok().body("")
}
