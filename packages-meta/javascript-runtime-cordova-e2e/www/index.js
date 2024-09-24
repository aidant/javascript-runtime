addEventListener('load', async () => {
  const id = crypto.randomUUID()
  await new Promise((resolve, reject) =>
    cordova.exec(resolve, reject, 'JavaScriptRuntimePlugin', 'start', [id, './native/index.js'])
  ).then(console.log, console.error)
  await new Promise((resolve, reject) =>
    cordova.exec(resolve, reject, 'JavaScriptRuntimePlugin', 'close', [id])
  ).then(console.log, console.error)
})
