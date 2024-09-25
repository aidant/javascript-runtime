import { JavaScriptRuntime } from '@javascript-runtime/cordova-plugin'
import { wrap } from 'comlink'

addEventListener('load', async () => {
  const native = await JavaScriptRuntime.import('./native.js')
  native.addEventListener('messageerror', console.error)
  const api = wrap(native)
  console.log(await api.fetch('https://1.1.1.1'))
})
