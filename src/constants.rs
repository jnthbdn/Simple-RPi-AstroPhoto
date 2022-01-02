pub static PATH_SAVE_CAPTURE: &str = "static/capture/";
pub static FILENAME_PREVIEW : &str = "static/catpure/preview.jpg";

// Env. variables
pub static ENV_SHOW_MMAL_ERROR : &str = "PRINT_MMAL_ERROR";

pub fn capture_path(filename : &str) -> String{
    format!("{}{}", PATH_SAVE_CAPTURE, filename)
}