class TwoPanelContainer{

    constructor(leftID, rightID, spacerID){
        this.leftPanel = document.getElementById(leftID);
        this.rightPanel = document.getElementById(rightID);
        this.spacer = document.getElementById(spacerID);

        this.isMoveSpacer = false;

        if( !this.leftPanel || !this.rightPanel || !this.spacer){
            console.error("Elements ID for TwoPanelContainer not founds");
            return;
        }

        this.spacer.addEventListener( "mousedown", (evt) => { this.isMoveSpacer = true; } );
        document.addEventListener( "mouseup", (evt) => { this.isMoveSpacer = false; } );
        document.addEventListener( "mousemove", (evt) => this.mouse_move(evt) );
    }


    mouse_move(evt){
        if( !this.isMoveSpacer ){ return; }

        let newPosX = Math.max( 50, Math.min(evt.clientX, document.body.clientWidth - 50));

        this.spacer.style.left = newPosX;
        this.leftPanel.style.width = newPosX;
        this.rightPanel.style.width = document.body.clientWidth - newPosX - this.spacer.clientWidth;
    }


}