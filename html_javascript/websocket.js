console.log('few')
const socket = new WebSocket('ws://localhost:8887')

socket.addEventListener('open', function(event) {
  console.log("open!!!!")
  socket.send('Hello Server!')
})

socket.addEventListener('message', function(event) {
  console.log('Message from server ', event.data)
})
