let modal;
let panelContainer;
let default_config = {};
let is_preview_enable = true;
let capture_table;

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

function shutterspeed_only_off(exposure_id, shutterspeed_id){

    let exposure = document.getElementById(exposure_id);
    let shutterspeed = document.getElementById(shutterspeed_id);

    shutterspeed.disabled = (exposure.value != "off");
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
    modal = new Modal("modal", "modal_title", "modal_text");

    add_preset_event("preset", "width", "height");

    document.getElementById("awb").addEventListener( "change", (evt) => awb_gain_only_off("awb", "awbblue", "awbred"));  
    document.getElementById("exposure").addEventListener( "change", (evt) => shutterspeed_only_off("exposure", "shutterspeed"));  

    document.querySelectorAll("[sendonchange]").forEach( (elem) => {

        if( elem.type == "range" ){
            add_event_slider(elem.id, elem.value);
        }

        if( elem.type == "text" || elem.type == "number"){
            elem.addEventListener("input", (evt) => {send_data(evt.target.id)});
        }
        else{
            elem.addEventListener("change", (evt) => {send_data(evt.target.id)});
        }
    });


    default_config = save_config(false);
    load_config();

    // Trigger 'input' event on awb to check value
    document.getElementById("awb").dispatchEvent(new Event('change'));
    document.getElementById("exposure").dispatchEvent(new Event('change'));
    table = new SimpleDataTable("capture_table");
}

function refresh_preview(preview_id){

    if( !is_preview_enable ){ return; }

    var date = new Date();
    var img_url = `/preview?id=${date.getSeconds()}-${date.getMilliseconds()}`;

    document.getElementById(preview_id).src=img_url;
}

async function take_photo(){
    var text = "Taking photo please wait...";
    var title = "Taking Photo";

    modal.showInfo(title, text);

    try{
        var resp = await fetch("/take_photo");
        var body = await resp.text();

        if( resp.ok ){
            text += `<br/><br/>Success : ${body}`;
            modal.showInfo(title, text);
        }
        else{
            text += `<br/><br/>Failure : ${resp.status} - ${body}`;
            modal.showError(title, text);
        }
    }
    catch(e){
        text += "<br/><br/>Failure : Internal Error.";
        console.error(e);
        modal.showError(title, text);
    }
}

async function take_video(){
    var duration = document.getElementById("duration_video");

    if( isNaN(duration.value) || duration.value < 1 ){
        duration.value = 1;
    }

    var text = `Taking video (during ${duration.value} second${duration.value > 1 ? "s" : ""}) please wait...`;
    var title = "Taking video";

    modal.showInfo(title, text);

    try{
        var resp = await fetch(`/take_video/${duration.value}`);
        var body = await resp.text();

        if( resp.ok ){
            text += `<br/><br/>Success : ${body}`;
            modal.showInfo(title, text);
        }
        else{
            text += `<br/><br/>Failure : ${resp.status} - ${body}`;
            modal.showError(title, text);
        }
    }
    catch(e){
        text += "<br/><br/>Failure : Internal Error.";
        console.error(e);
        modal.showError(title, text);
    }
}

function toggle_preview(enable){
    is_preview_enable = enable;
}

function toggle_crosshair(enable){
    if(enable){
        console.log("Enable crosshair");
    }
    else{
        console.log("Disable crosshair");
    }
}

function change_preview_fit(classname){
    document.getElementById("preview").className = classname;
}

async function open_capture_folder(){

    fetch("/list_capture")
    .then(result => {

        if( !result.ok ){
            modal.showError("Failed to load capture", "Unable to load capture data : " + e);
            return;
        }

        result.json()
        .then( (json) => {

            var tbody = document.getElementById("capture_table").querySelector("tbody");
            tbody.innerHTML = "";

            for( const line of json ){
                var tr = document.createElement("tr");

                tr.innerHTML = `<td><input type="checkbox" filepath="${line["href"]}"/></td>`;
                tr.innerHTML += `<td><img src="${line["href"]}" width="200px"/></td>`;
                tr.innerHTML += `<td>${line["filename"]}</td>`;
                tr.innerHTML += `<td>${line["date"]}</td>`;
                tr.innerHTML += `<td>${line["size"]}</td>`;
                tr.innerHTML += `<td>${line["file_type"]}</td>`;
                tr.innerHTML += `<td><button class="primary-button" onclick="download_file('${line["href"]}', '${line["filename"]}')">Download</button><button onclick="open_file('${line["href"]}')">Open</button><button class="danger-button" onclick="delete_file('${line["href"]}')">Delete</button></td>`;

                tbody.append(tr);
            }

            document.getElementById("capture_folder").style.display = "block";
        })
        .catch((e) => {
            modal.showError("Failed to parse capture inforamtions", "Unable to parse capture datas : " + e);
        });
    })
    .catch( (e) => {
        modal.showError("Failed to load capture", "An error occured : " + e);
    });
}

function close_capture_folder(){
    document.getElementById("capture_folder").style.display = "none";
}

function download_file(path){

    let filename = path.substring(path.lastIndexOf("/") + 1)

    // var a = document.createElement("a");
    // a.href = path;
    // a.setAttribute("download", filename);

    // document.body.appendChild(a);
    // a.click();
    // document.body.removeChild(a);

    var xhr = new XMLHttpRequest()
    xhr.open("GET", path)
    xhr.responseType = 'blob'
    xhr.onload = function() {
        saveAs(xhr.response, filename);
    }
    xhr.send()

}

function open_file(path){
    window.open(path, '_blank').focus();
}

async function delete_file(path){
    let file = path.substring(path.lastIndexOf("/") + 1)
    if( !confirm(`Are you sure you want to delete "${file}" ? You can't undo this action.`) ){
        return;
    }

    let del = await request_delete_file(file);
    if( del === true ){
        open_capture_folder();
    }
    else{
        modal.showError("Delete file", del);
    }
}

function toggle_all_selection(){

    let selection = document.getElementById("capture_folder").querySelectorAll("input[type=checkbox]");
    let is_all_selected = document.getElementById("capture_folder").querySelectorAll("input[type=checkbox]:checked").length == selection.length;
    
    for( const sel of selection ){
            sel.checked = is_all_selected ? false : true;
    }
}

async function download_selection(){

    let selection = document.getElementById("capture_folder").querySelectorAll("input[type=checkbox]:checked");

    if( selection.length == 0 ){
        showInfo("Download selection", "The selection is empty, nothing to download.");
        return;
    }

    let zip = new JSZip();

    for( const sel of selection ){
        let path = sel.getAttribute("filepath");
        let filename = path.substring(path.lastIndexOf("/") + 1)

        try{
            
            let result = await fetch(path);
            
            if( !result.ok ){ continue; }

            let blob =  result.blob();

            zip.file( filename, blob, {binary: true, } );
        }
        catch(e){
            console.error(e);
        }
    }

    zip.generateAsync({type: "blob"}).then(zipfile => saveAs(zipfile, "rpi_astro.zip"));
}

async function delete_selection(){

    let selection = document.getElementById("capture_folder").querySelectorAll("input[type=checkbox]:checked");
    let errors = [];

    if( selection.length == 0){return;}

    if( !confirm(`Are you sure you want to delete the selection ? You can't undo this action.`) ){
        return;
    }


    for( const sel of selection ){
        let path = sel.getAttribute("filepath");
        let filename = path.substring(path.lastIndexOf("/") + 1)

        let del = await request_delete_file(filename);

        if( del !== true ){
            errors.push({file: filename, error: del});
        }
    }

    if( errors.length > 0 ){
        let err = "";

        for( const e of errors ){
            err += `    - ${e['file']} : ${e['error']}<br />`;
        }

        modal.showError("Delete selection", "Some files can't be deleted : <br/>" + err);
    }
    
    open_capture_folder();
}

async function request_delete_file(file){
    try{
        let result = await fetch(`/delete/${file}`, {method: "DELETE"});

        if( result.ok ) {
            return true;
        }

        if( result.status == 404 ){
            return `The file "${file}" was not found...`;
        }
        else{
            let body = await response.text();
            return `Unable to delete "${file}": ${body}`;
        }
    }
    catch(e){
        return `An error occured during the request : "${e}"`;
    }
}
