use std::result::Result;
use std::process::{Command, Child, Stdio};
use std::env;

use crate::constants::*;

pub struct RpiCam {
    pub width: u32,
    pub height: u32,

    pub hflip: bool,
    pub vflip: bool,
    pub rotation: u32,

    pub shutter_speed: u32,
    pub sharpness: i8,
    pub contrast: i8,
    pub brightness: i8,
    pub saturation: i8,
    pub iso: u16,
    pub stabilization: bool,
    pub ev_compensation: i8,
    pub exposure: String,
    pub analog_gain: f32,
    pub digital_gain: f32,
    pub awb: String,
    pub awb_blue: f32,
    pub awb_red: f32,
    pub effect: String,
    pub metering: String,
    pub drc: String,

    preview_process: Option<Child>
    
}

impl RpiCam{

    pub fn new() -> RpiCam{
        RpiCam{
            width: 1920,
            height: 1080,

            hflip: false,
            vflip: false,
            rotation: 0,

            shutter_speed: 0,
            sharpness: 0,
            contrast: 0,
            brightness: 50,
            saturation: 0,
            iso: 800,
            stabilization: false,
            ev_compensation: 0,
            exposure: String::from("auto"),
            analog_gain: 1.0,
            digital_gain: 1.0,
            awb: String::from("auto"),
            awb_blue: 1.0,
            awb_red: 1.0,

            effect: String::from("none"),
            metering: String::from("average"),
            drc: String::from("off"),

            preview_process: None
        }
    }

    pub fn take_photo(&mut self, filename : &str) -> Result<(), String>{

        let mut command = self.generate_raspi_command("raspistill", 1);
        command.args(&["-o", filename]);

        println!("{:#?}", command);

        self.stop_preview();
        let result = command.output();
        self.start_preview();

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("[take_photo error] {}", e))
        }
    }

    pub fn start_preview(&mut self) {

        if self.preview_process.is_some() {
            return;
        }

        let mut cmd = self.generate_raspi_command("raspistill", 0);
        cmd.args(&["-tl", "500", "-o", FILENAME_PREVIEW]);

        if env::var(ENV_SHOW_MMAL_ERROR).is_err() {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }

        self.preview_process = Some( cmd.spawn().expect("Failed to start rpistill"));

    }

    pub fn stop_preview(&mut self){
        
        if self.preview_process.is_some() {
            self.preview_process.as_mut().unwrap().kill().unwrap_or(());
            self.preview_process = None;
        }
    }

    pub fn restart_preview(&mut self){
        self.stop_preview();
        self.start_preview();
    }

    pub fn check_preview_status(&mut self){
        if self.preview_process.is_none() { return; }


        if RpiCam::is_running_pid(self.preview_process.as_mut().unwrap().id()) {
            return;
        }

        self.restart_preview();
    }

    fn is_running_pid(pid: u32) -> bool{
        let pid = Command::new("ps").args(&["--pid", &(pid.to_string()), "-o", "stat="]).output();

        if pid.is_err() {
            eprintln!("IS_RUNNING_PID ERROR : {}", pid.unwrap_err());
            return false;
        }

        let output = pid.unwrap();

        if output.status.code().is_none() || output.status.code().unwrap() != 0 || output.stdout.len() == 0 { return false; }

        let first_letter = output.stdout[0] as char;

        return first_letter == 'S' || first_letter == 'R';
    }

    fn generate_raspi_command(&self, cmd_name: &str, timeout: u8) -> Command{
        let mut cmd = Command::new(cmd_name);
        

        // Rotation force to change the size of the picture or raspistill will crop it. 
        // See: https://forums.raspberrypi.com/viewtopic.php?t=47650#p533620
        match self.rotation {
            0 | 180 => {
                cmd.arg("-w").arg(self.width.to_string())
                    .arg("-h").arg(self.height.to_string());
            }
            90 | 270 => {
                cmd.arg("-w").arg(self.height.to_string())
                    .arg("-h").arg(self.width.to_string());
            }
            _ => ()
        };

        cmd.arg("-rot").arg(self.rotation.to_string())
            .arg("-sh").arg(self.sharpness.to_string())
            .arg("-co").arg(self.contrast.to_string())
            .arg("-br").arg(self.brightness.to_string())
            .arg("-sa").arg(self.saturation.to_string())
            .arg("-ISO").arg(self.iso.to_string())

            .arg("-ex").arg(self.exposure.clone())
            .arg("-awb").arg(self.awb.clone())
            .arg("-ifx").arg(self.effect.clone())
            .arg("-mm").arg(self.metering.clone())
            .arg("-drc").arg(self.drc.clone())

            .arg("-ev").arg(self.ev_compensation.to_string())
            .arg("-ag").arg(self.analog_gain.to_string())
            .arg("-dg").arg(self.digital_gain.to_string())
            
            .arg("-t").arg(timeout.to_string());

        if self.hflip { cmd.arg("-hf"); }
        if self.vflip { cmd.arg("-vf"); }
        if self.shutter_speed > 0 { cmd.arg("-ss").arg((self.shutter_speed * 1000).to_string()); }
        if self.stabilization { cmd.arg("-vs"); }
        if self.awb == "off" { cmd.arg("-awbg").arg(format!("{},{}", self.awb_blue, self.awb_red)); }

        return cmd;
    }
}