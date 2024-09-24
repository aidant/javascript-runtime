import {
  init,
  JavaScriptRuntime,
  type JavaScriptRuntimeOps,
  type JSONValue,
} from '@javascript-runtime/webview'

declare const cordova: {
  exec: (
    successFunction: (result?: unknown) => void,
    failFunction: (message: string) => void,
    service: string,
    action: string,
    args?: unknown[]
  ) => void
}

init(
  Object.fromEntries(
    ['start', 'close', 'postMessage', 'pollDispatchEvent'].map((action) => [
      action,
      (...args: unknown[]) =>
        new Promise((resolve, reject) =>
          cordova.exec(
            resolve,
            (message) => reject(new Error(message)),
            'JavaScriptRuntimePlugin',
            action,
            args
          )
        ),
    ])
  ) as object as JavaScriptRuntimeOps
)

export { JavaScriptRuntime, type JSONValue }
