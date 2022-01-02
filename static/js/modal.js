class Modal{
    constructor( modal_id, title_id, text_id ){
        this.modal = document.getElementById(modal_id);
        this.title = document.getElementById(title_id);
        this.text = document.getElementById(text_id);
        this.isOpen = false;

        window.addEventListener('keyup', (e) => this._keyEvent(e));
    }

    showError(title, text){
        this.modal.className = "danger";
        this._setTexts(title, text);
        this.open();
    }

    showInfo(title, text){
        this.modal.className = "info";
        this._setTexts(title, text);
        this.open();
    }

    close(){
        this.modal.style.display = "none";
        this.isOpen = false;
    }

    open(){
        this.modal.style.display = "block";
        this.isOpen = true;
    }

    _setTexts(title, text){
        this.title.innerHTML = title;
        this.text.innerHTML = text;
    }

    _keyEvent(e){
        if(e.keyCode == 27 && this.isOpen ){
            this.close();
        }
    }
}