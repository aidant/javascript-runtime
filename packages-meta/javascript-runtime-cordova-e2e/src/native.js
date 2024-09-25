import { expose } from 'comlink'

console.log('hello world')

JavaScriptRuntime.addEventListener('message', (event) => console.log('event', event))

JavaScriptRuntime.postMessage({ hello: 'world' })

expose(
  {
    fetch: (url) => fetch(url).then((response) => response.text()),
  },
  JavaScriptRuntime
)
