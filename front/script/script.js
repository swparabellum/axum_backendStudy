function clickBtn(getColor){
    if(getColor =='yellow'){
        document.body.style.backgroundColor = "yellow";
    }else if(getColor == 'white'){
        document.body.style.backgroundColor = "white";
    }
}
var cnt =0;
function counter(){
    cnt++;
    if(cnt == 3){
alert("3번 누르셨습니다!");
cnt =0;
    }
}
function counter2(type){
    const myCounter = document.getElementById('getCount');
    let number = myCounter.innerText;

    if(type == 'plus'){
        number = parseInt(number) +1;
    }
    myCounter.innerText=number;
}



function changeBackgroundColor() { 
    var x = Math.floor(Math.random() *256), 
        y = Math.floor(Math.random() * 256),
        z = Math.floor(Math.random() * 256);
        //R, G, B

        document.body.style.backgroundColor = 'rgb(' + x + ',' + y + ',' + z + ')';
        //https://stackoverflow.com/questions/64089791/button-to-change-background-color-randomly
};

function json_submit() {
    let obj = {
        email: document.getElementById("id").value,
        password: document.getElementById("password").value,
    };
    console.log(obj);

    let xmlhttp = new XMLHttpRequest();
    xmlhttp.open("POST","api/login");
    xmlhttp.setRequestHeader("Content-type", "application/json");
    xmlhttp.send(JSON.stringify(obj));

}
