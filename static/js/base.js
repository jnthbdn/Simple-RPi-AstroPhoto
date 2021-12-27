let panelContainer;

function evt_change(elem, targetID){
    let field = document.getElementById(targetID);

    if( parseFloat(elem.value) > parseFloat(elem.max) ){ elem.value = elem.max; }
    if( parseFloat(elem.value) < parseFloat(elem.min) ){ elem.value = elem.min; }

    if(!field || !elem){ return; }

    field.value = elem.value;
}

function add_event_slider(slider_id, default_value){
    let field_id = slider_id + "-value";
    
    document.getElementById(slider_id).addEventListener("input", (evt) => evt_change(evt.target, field_id));
    document.getElementById(field_id).addEventListener("input", (evt) => {
        evt_change(evt.target, slider_id);
        send_data(slider_id);
    });
    document.getElementById(slider_id).addEventListener("dblclick", (evt) => { 
        document.getElementById(slider_id).value = default_value;
        document.getElementById(field_id).value = default_value;
        send_data(slider_id);
     });
}

function add_preset_event(preset_id, width_id, height_id){
    document.getElementById(preset_id).addEventListener("input", (evt) => {
        
        let width = document.getElementById(width_id);
        let height = document.getElementById(height_id);
    
        switch( evt.target.value ){
            case "2160":
                width.value = 3840;
                height.value = 2160;
                break;

            case "1440":
                width.value = 2560;
                height.value = 1440;
                break;

            case "1080":
                width.value = 1920;
                height.value = 1080;
                break;

            case "720":
                width.value = 1280;
                height.value = 720;
                break;

            case "480":
                width.value = 854;
                height.value = 480;
                break;

            case "360":
                width.value = 640;
                height.value = 360;
                break;

            case "240":
                width.value = 426;
                height.value = 240;
                break;
            
            default:
                break;
        }

        send_data(width_id);
        send_data(height_id);
    });
}

function awb_gain_only_off(awb_id, awbblue_id, awbred_id){

    let awb = document.getElementById(awb_id);
    let awbblue = document.getElementById(awbblue_id);
    let awbred = document.getElementById(awbred_id);
    let awbblue_value = document.getElementById(awbblue_id + "-value");
    let awbred_value = document.getElementById(awbred_id + "-value");


    if( awb.value == "off" ){
        awbblue.disabled = false;
        awbred.disabled = false;
        awbblue_value.disabled = false;
        awbred_value.disabled = false;
    }
    else{
        awbblue.disabled = true;
        awbred.disabled = true;
        awbblue_value.disabled = true;
        awbred_value.disabled = true;
    }
}


function send_data(id){
    let elem = document.getElementById(id);
    let value = null;

    switch(elem.type){
        case "checkbox":
            value = elem.checked ? 1 : 0;
            break;

        case "range":
        case "number":
            value = parseFloat(elem.value);
            break;

        default:
            value = elem.value;
            break;
    }

    console.log(`Send data of '${id}' (${elem.type}) with value '${value}'`)
}

function init_base(){
    panelContainer = new TwoPanelContainer("left_container", "right_container", "spacer_container");


    add_preset_event("preset", "width", "height");

    add_event_slider("rotation", 0);
    add_event_slider("sharpness", 0);
    add_event_slider("contrast", 0);
    add_event_slider("brightness", 50);
    add_event_slider("saturation", 0);
    add_event_slider("iso", 800);
    add_event_slider("evcompensation", 0);

    add_event_slider("awbblue", 1);
    add_event_slider("awbred", 1);
    add_event_slider("digitalgain", 1);
    add_event_slider("analoggain", 1);

    awb_gain_only_off("awb", "awbblue", "awbred");
    document.getElementById("awb").addEventListener( "input", (evt) => awb_gain_only_off("awb", "awbblue", "awbred"));  

    document.querySelectorAll("[sendonchange]").forEach( (elem) => {
        elem.addEventListener("input", (evt) => {send_data(evt.target.id)});
    });

}
