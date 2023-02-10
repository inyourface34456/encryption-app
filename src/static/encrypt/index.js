document.getElementById("submit").onclick = function() {
    var request = new XMLHttpRequest();
    var params = document.getElementById('myTextarea').value;
    request.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            document.getElementById('link').innerHTML = `${this.responseText}`;
        }
    };
    request.open('POST', 'http://localhost:8000/encrypt', true);
    request.send(params);
};