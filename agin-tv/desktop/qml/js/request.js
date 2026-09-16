function sendRequest(url, token, callback) {
  let request = new XMLHttpRequest();

  request.open("GET", url);
  request.setRequestHeader("Authorization", `Bearer ${token}`);
  request.onreadystatechange = function () {
    if (request.readyState === XMLHttpRequest.DONE) {
      let response = {
        status: request.status,
        headers: request.getAllResponseHeaders(),
        contentType: request.responseType,
        content: request.response,
      };

      callback(response);
    }
  };

  request.send();
}
