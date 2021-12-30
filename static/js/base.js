let panelContainer;
let default_config = {};

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

    if( !elem ){
        console.error(`'${id}' element not found !`);
        return;
    }

    switch(elem.type){
        case "checkbox":
            value = elem.checked ? "true" : "false";
            break;

        case "range":
        case "number":
            value = parseFloat(elem.value);
            break;

        default:
            value = elem.value;
            break;
    }

    fetch(`/${id}/${value}`, {method: "POST"})
        .then( (response) => response.text() )
        .then( (response) => console.log(response))
        .catch( (error) => console.error(`Fetch error [/${id}/${value}] : ${error}`));

    save_config();
}

function save_config(saveOnLocalStorage = true){

    let config = {};

    document.querySelectorAll("[sendonchange]").forEach( (elem) => {
        
        if(elem.type == "checkbox"){
            config[elem.id] = elem.checked;
        }
        else{
            config[elem.id] = elem.value;
        }

    });

    if(saveOnLocalStorage)
        localStorage.setItem("config", JSON.stringify(config));
    else
        return config;
}

function load_config(){

    let rawConfig = localStorage.getItem("config");
    let config;

    try{
        config = JSON.parse(rawConfig);
    }
    catch{
        return;
    }

    if( config == false || config == null ){ return; }


    document.querySelectorAll("[sendonchange]").forEach( (elem) => {
        
        if( elem.id in config )
        {    
            switch( elem.type ){
                case "range":
                    elem.value = config[elem.id];
                    document.getElementById(elem.id + "-value").value = config[elem.id];
                    break;

                case "checkbox":
                    elem.checked = config[elem.id];
                    break;

                default:
                    elem.value = config[elem.id];
                    break;
            }

            send_data(elem.id);
        }
    });
}

function reset_config(){

    if(!confirm("Reset ALL the parameters? This action cannot be undone!")){
        return;
    }

    localStorage.setItem("config", JSON.stringify(default_config));
    load_config();
}

function init_base(){
    panelContainer = new TwoPanelContainer("left_container", "right_container", "spacer_container");


    add_preset_event("preset", "width", "height");
    awb_gain_only_off("awb", "awbblue", "awbred");

    document.getElementById("awb").addEventListener( "input", (evt) => awb_gain_only_off("awb", "awbblue", "awbred"));  

    document.querySelectorAll("[sendonchange]").forEach( (elem) => {

        if( elem.type == "range" ){
            add_event_slider(elem.id, elem.value);
        }

        if( elem.type == "text")
            elem.addEventListener("input", (evt) => {send_data(evt.target.id)});
        else
            elem.addEventListener("change", (evt) => {send_data(evt.target.id)});
    });


    default_config = save_config(false);
    load_config();
}

function refresh_preview(preview_id){
    var date = new Date();
    var url = "/take_photo"
    var img_url = `/preview?id=${date.getHours()}-${date.getMinutes()}-${date.getSeconds()}-${date.getMilliseconds()}`;

    document.getElementById(preview_id).src=img_url;
    setTimeout(() => refresh_preview(preview_id), 500);
    

    // fetch(url, {method: "GET"})
    //     .then( (response) => {
    //         if(response.status == 200 ){
    //             document.getElementById(preview_id).src=img_url;
    //             setTimeout(() => refresh_preview(preview_id), 100);
    //         }
    //         else{
    //             setTimeout(() => refresh_preview(preview_id), 1000);
    //         }
    //     })
    //     .catch( (error) => {
    //         console.error(`No preview image available`);
    //         setTimeout(() => refresh_preview(preview_id), 1000);
    //     });
}
