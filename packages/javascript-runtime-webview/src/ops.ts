export type JSONValue = null | string | number | boolean | { [x: string]: JSONValue } | JSONValue[]

export const EventConstructor = {
  Event,
  ErrorEvent,
  CloseEvent,
  MessageEvent,
  CustomEvent,
} as const

export interface JavaScriptRuntimeOps {
  start: (id: string, specifier: string) => Promise<void>
  close: (id: string) => Promise<void>
  postMessage: (id: string, message: JSONValue) => Promise<void>
  pollDispatchEvent: (id: string) => Promise<
    | {
        [Constructor in keyof typeof EventConstructor]: {
          constructor: Constructor
          type: ConstructorParameters<(typeof EventConstructor)[Constructor]>['0']
          eventInitDict: Exclude<
            ConstructorParameters<(typeof EventConstructor)[Constructor]>['1'],
            undefined
          >
        }
      }[keyof typeof EventConstructor]
    | null
  >
}

export let ops: JavaScriptRuntimeOps

export const init = (_ops: JavaScriptRuntimeOps) => {
  ops = _ops
}
