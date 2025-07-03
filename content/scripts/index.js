class Error {
    constructor(message) {
        this.message = message;
        this.presence = message !== "";
    }
}

var error = new Error("");

async function getErrorPresence() {
    console.log("test");
    let response = await fetch(window.location.href + "get_error_presence");
    let data = await response.json();
    console.log(data);
    error = new Error(data.message);
    return error.presence;
}

function updateErrorMessage() {
    document.getElementById('error_message').innerHTML = error.message;
    document.getElementById('error_div').style.display = error.presence ? "block" : "none";
}

function moveMotor(motor) {
    const data = document.getElementById('nb_field').value;
    console.log(motor, data);
    fetch(window.location.href + "send_pos", {
        method: "POST",
        body: JSON.stringify({
            motor: motor,
            angle: data
        }),
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    });
}

function init() {
    getErrorPresence().then(r => updateErrorMessage());
}

init();
