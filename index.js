const app = require('./app')
const { LISTEN_FD, PORT } = process.env

if (LISTEN_FD) {
  app.listen({ fd: +LISTEN_FD }, () =>
    console.log('Listening on socket')
  )
} else {
  const server = app.listen(PORT || 5000, () =>
    console.log(`Listening on port ${server.address().port}`)
  )
}
