pub static PATH_SAVE_CAPTURE: &str = "static/capture/";
pub static HREF_CAPTURE_PATH : &str = "/capture/";
pub static FILENAME_PREVIEW : &str = "static/capture/preview.jpg";

// Env. variables
pub static ENV_SHOW_MMAL_ERROR : &str = "PRINT_MMAL_ERROR";
pub static ENV_DISABLE_PREVIEW : &str = "DISABLE_PREVIEW";

pub fn capture_path(filename : &str) -> String{
    format!("{}{}", PATH_SAVE_CAPTURE, filename)
}