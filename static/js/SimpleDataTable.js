class SimpleDataTable{
    static CLASS_TABLE = "sdt-table";
    static CLASS_SORTABLE = "sdt-sortable";
    static CLASS_SORT_ASC = "sdt-sort-asc";
    static CLASS_SORT_DESC = "sdt-sort-desc";

    constructor(table_id){
        this.table = document.getElementById(table_id);
        this.table.classList.add(SimpleDataTable.CLASS_TABLE);

        this._addEventToHeader();
    }
     

    sortByColumn(column_id){
        console.log("Sort By column : " + column_id);

        let columns = this._getColumns();

        if( column_id >= columns.length ){
            console.error("[sortByColumn] column ID is not valid !");
            return;
        }

        let target_column = columns[column_id];
        let is_sort_asc;


        if( target_column.classList.contains(SimpleDataTable.CLASS_SORT_ASC) ){
            this._clearSortClass();
            target_column.classList.remove(SimpleDataTable.CLASS_SORT_ASC);
            target_column.classList.add(SimpleDataTable.CLASS_SORT_DESC);
            is_sort_asc = false; 
        }
        else{
            this._clearSortClass();
            target_column.classList.remove(SimpleDataTable.CLASS_SORT_DESC);
            target_column.classList.add(SimpleDataTable.CLASS_SORT_ASC);
            is_sort_asc = true; 
        }

        let data = this._getTableData();
        data.sort( (a, b) => {
            if( a[column_id] == b[column_id] ){ return 0; }

            if( is_sort_asc ){
                if( a[column_id] < b[column_id] ){ return -1; }
                else { return 1;}
            }
            else{
                if( a[column_id] < b[column_id] ){ return 1; }
                else { return -1;}
            }
        });

        this._setTableData(data);

    }

    _addEventToHeader(){
        let elements = this._getColumns();

        for( let i = 0; i < elements.length; ++i ){
            if( elements[i].attributes.getNamedItem("unsortable")){ continue; }

            elements[i].addEventListener("click", (e) => this.sortByColumn(i));
            elements[i].classList.add(SimpleDataTable.CLASS_SORTABLE);
        }
    }

    _getColumns(){
        return this.table.querySelectorAll("thead tr td");
    }

    _getTableData(){
        let result = [];

        for( const tr of this.table.querySelectorAll("tbody tr") ){
            let tr_data = {};

            let tds = tr.querySelectorAll("td");
            for( var i = 0; i < tds.length; ++i ){
                tr_data[i] = tds[i].innerText;
            }

            result.push(tr_data);
        }


        return result;
    }

    _setTableData(data){

        let tbody = this.table.querySelector("tbody");
        tbody.innerHTML = "";

        for(const d of data ){

            let tr = document.createElement("tr");

            for( const key in d ){
                let td = document.createElement("td");
                td.innerText = d[key];
                tr.append(td);
            }

            tbody.append(tr);
        }
    }

    _clearSortClass(){
        for( const c of this._getColumns() ){
            c.classList.remove(SimpleDataTable.CLASS_SORT_ASC);
            c.classList.remove(SimpleDataTable.CLASS_SORT_DESC);
        }
    }
}