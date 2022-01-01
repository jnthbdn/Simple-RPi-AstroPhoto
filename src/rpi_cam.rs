use std::result::Result;
use std::process::{Command, Child, Stdio};
use std::fs;

struct PreviewProcesses {
    raspivid: Child,
    ffmpeg: Child
}

impl PreviewProcesses {
    pub fn new(raspivid: Child, ffmpeg: Child) -> PreviewProcesses{
        PreviewProcesses{
            raspivid: raspivid,
            ffmpeg: ffmpeg
        }
    }

    pub fn kill(&mut self){
        self.ffmpeg.kill().expect("Failed to kill ffmpeg process !");
        self.raspivid.kill().expect("Failed to kill Raspivid process !");
    }
}

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

    preview_process: Option<PreviewProcesses>
    
}


pub static FILENAME_PREVIEW : &str = "static/img/preview.jpg";

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

    pub fn take_pic(&self, filename : &str) -> Result<(), String>{

        let mut command = self.generate_raspi_command("raspistill", 1);
        command.args(&["-o", filename]);

        println!("{:#?}", command);

        match command.output() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("ERROR: {}", e))
        }
    }

    pub fn start_preview(&mut self) {

        if self.preview_process.is_some() {
            println!("Preview already started !");
            return;
        }

        self.delete_preview_file();

        let mut rpivid = self.generate_raspi_command("raspivid", 0)
                             .args(&["-o", "-"])
                             .stdout(Stdio::piped())
                             .spawn().expect("Failed to start raspivid !");

        let ffmpeg = Command::new("ffmpeg")
                                    .args(&["-i", "pipe:", "-update", "1", FILENAME_PREVIEW])
                                    .stdin(rpivid.stdout.take().unwrap())
                                    .spawn();

        match ffmpeg{
            Ok(_) => self.preview_process = Option::from(PreviewProcesses::new(rpivid, ffmpeg.unwrap())),
            Err(e) => {
                rpivid.kill().expect("Failed to kill raspivid !");
                panic!("Failed to start ffmpeg !\n{}", e);
            }
        };
        
        // INFO : raspivid -t 0 -o - | ffmpeg -i pipe: -update 1 plop.jpg
    }

    pub fn stop_preview(&mut self){
        if self.preview_process.is_some() {
            println!("Stop preview");
            self.preview_process.as_mut().unwrap().kill();
            self.preview_process = None;
        }
    }

    fn generate_raspi_command(&self, cmd_name: &str, timeout: u8) -> Command{
        let mut cmd = Command::new(cmd_name);
        
        
        cmd.arg("-w").arg(self.width.to_string())
            .arg("-h").arg(self.height.to_string())
            .arg("-rot").arg(self.rotation.to_string())

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

    #[allow(dead_code)]
    fn generate_raspi_command_string(&self, cmd_name: &str, timeout: u8) -> String {

        let mut cmd = String::from(cmd_name);
        
        cmd += " ";
        cmd += &format!("-w {} ", self.width);

            cmd += &format!("-h {} ", self.height.to_string());
            cmd += &format!("-rot {} ", self.rotation.to_string());

            cmd += &format!("-sh {} ", self.sharpness.to_string());
            cmd += &format!("-co {} ", self.contrast.to_string());
            cmd += &format!("-br {} ", self.brightness.to_string());
            cmd += &format!("-sa {} ", self.saturation.to_string());
            cmd += &format!("-ISO {} ", self.iso.to_string());

            cmd += &format!("-ex {} ", self.exposure.clone());
            cmd += &format!("-awb {} ", self.awb.clone());
            cmd += &format!("-ifx {} ", self.effect.clone());
            cmd += &format!("-mm {} ", self.metering.clone());
            cmd += &format!("-drc {} ", self.drc.clone());

            cmd += &format!("-ev {} ", self.ev_compensation.to_string());
            cmd += &format!("-ag {} ", self.analog_gain.to_string());
            cmd += &format!("-dg {} ", self.digital_gain.to_string());
            
            cmd += &format!("-t {} ", timeout);

        if self.hflip { cmd += "-hf "; }
        if self.vflip { cmd += "-vf "; }
        if self.shutter_speed > 0 { cmd += &format!("-ss {} ", (self.shutter_speed * 1000).to_string()); }
        if self.stabilization { cmd += "-vs "; }
        if self.awb == "off" { cmd += &format!("-awbg {},{} ", self.awb_blue, self.awb_red); }

        return cmd;
    }

    fn delete_preview_file(&self){
        match fs::remove_file(FILENAME_PREVIEW){
            Ok(_) => (),
            Err(e) => eprintln!("{}", e)
        };
    }
}