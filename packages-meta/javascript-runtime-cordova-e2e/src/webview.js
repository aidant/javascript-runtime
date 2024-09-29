import { JavaScriptRuntime } from '@javascript-runtime/cordova-plugin'
import { wrap } from 'comlink'

window.JavaScriptRuntime = JavaScriptRuntime
window.Comlink = { wrap }

addEventListener('load', async () => {
  const native = await JavaScriptRuntime.import('./native.js')
  native.addEventListener('error', console.error)
  native.addEventListener('messageerror', console.error)
  const api = wrap(native)
  console.log(await api.fetch('https://1.1.1.1'))
  await native.close()
})
