function show_board(){
    const board = document.querySelector(".board")

    for(let i = 0; i<9; i++){
        let row = document.createElement("div")
        row.className = "row"
        board.appendChild(row)

        for(let j = 0; j<9; j++){
            let cell = document.createElement("input")
            cell.type = "text"
            cell.className = "cell"
            row.appendChild(cell)
        }
    }   
}

show_board()

function build(){
    const board = document.querySelector(".board")

    string = ""

    for(let i = 0; i<9;i++){
        let row = board.childNodes
        let cell = row[i].childNodes

        string += "["

        for(let j = 0; j<9;j++){
            if(cell[j].value === "") cell[j].value = 0
            
            if(j == 8) string += `${cell[j].value}`
            else string += `${cell[j].value},`
        } 

        if(i == 8) string += "]\r\n"
        else string += "],\r\n"
    }
    console.log(string)
}

function clear_build(){
    const board = document.querySelector(".board")

    for(let i = 0; i<9;i++){
        let row = board.childNodes
        let cell = row[i].childNodes

        for(let j = 0; j<9;j++){
            cell[j].value = ""
        } 
    }
}