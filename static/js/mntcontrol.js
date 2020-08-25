function setLight(name, brightness) {
  fetch('/lights/'+name, {
    method: 'put',
    headers: new Headers({'Content-type': 'application/json'}),
    body: JSON.stringify({
      brightness: brightness
    })
  }).then(function(response) {
    return response.json()
  }).then(function(data) {
    console.log('setLight response:', data)
  })
}
