use std::result::Result;
use std::process::Command;

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
    pub drc: String
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
            drc: String::from("off")
        }
    }

    pub fn take_pic(&self, filename : &str) -> Result<(), String>{

        let mut command = Command::new("raspistill");
                        
        command.arg("-w").arg(self.width.to_string())
               .arg("-h").arg(self.height.to_string())
               .arg("-o").arg(filename.clone())
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
               .arg("-dg").arg(self.digital_gain.to_string());

        if self.hflip { command.arg("-hf"); }
        if self.vflip { command.arg("-vf"); }
        if self.shutter_speed > 0 { command.arg("-ss").arg((self.shutter_speed * 1000).to_string()); }
        if self.stabilization { command.arg("-vs"); }
        if self.awb == "off" { command.arg("-awbg").arg(format!("{},{}", self.awb_blue, self.awb_red)); }

        command.arg("-t").arg("1");

        println!("{:#?}", command);

        match command.output() {
            Ok(_) => (),
            Err(e) => println!("ERROR: {}", e)
        }


        Ok(())
    }



}