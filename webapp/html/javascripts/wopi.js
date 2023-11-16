const graphics = [
    "/images/one.png",
    "/images/two.png",
];
let current_graphics = graphics[0];

function post(msg) {
    window.document.querySelector("#collabora-online-viewer").contentWindow.postMessage(JSON.stringify(msg), '*');
}

function postReady() {
    post({
        MessageId: 'Host_PostmessageReady'
    });
}

function receiveMessage(event) {
    console.log('==== receiveMessage: ' + event.data);
    let msg = JSON.parse(event.data);
    if (!msg) {
        return;
    }
    if (msg.MessageId == 'App_LoadingStatus') {
        if (msg.Values) {
            if (msg.Values.Status == 'Document_Loaded') {
                post({'MessageId': 'Host_PostmessageReady'});
            }
        }
    } else if (msg.MessageId === "UI_InsertGraphic") {
        post({
            MessageId: "Action_InsertGraphic",
            Values: {
                url: current_graphics
            }
        });
    }

    console.log(msg.MessageId);
}

function select_graphics(index) {
    current_graphics = document.location.origin + graphics[index];
}

function select_remote_graphic(url) {
    current_graphics = url;
}

window.addEventListener("message", receiveMessage, false);
