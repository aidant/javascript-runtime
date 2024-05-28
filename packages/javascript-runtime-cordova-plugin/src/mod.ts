import {
  JavaScriptRuntime,
  init,
  type JSONValue,
  type JavaScriptRuntimeOps,
} from '@javascript-runtime/webview'

declare const cordova: {
  exec: (
    successFunction: Function,
    failFunction: Function,
    service: string,
    action: string,
    args?: unknown[]
  ) => void
}

const exec = <Action extends keyof JavaScriptRuntimeOps>(
  action: Action,
  args: Parameters<JavaScriptRuntimeOps[Action]>
): Promise<Awaited<ReturnType<JavaScriptRuntimeOps[Action]>>> =>
  new Promise((resolve, reject) =>
    cordova.exec(
      resolve,
      (message: string) => reject(new Error(message)),
      'JavaScriptRuntime',
      action,
      args
    )
  )

init({
  start: (id: string, specifier: string) => exec('start', [id, specifier]),
  close: (id: string) => exec('close', [id]),
  postMessage: (id: string, message: JSONValue) => exec('postMessage', [id, message]),
  pollDispatchEvent: (id: string) => exec('pollDispatchEvent', [id]),
})

export { JavaScriptRuntime, type JSONValue }
