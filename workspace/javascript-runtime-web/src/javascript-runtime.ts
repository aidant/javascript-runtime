import { EventConstructor, ops, type JSONValue } from './ops.js'
import { run, unwrap } from './utils.js'

export class JavaScriptRuntime extends EventTarget {
  #id = crypto.randomUUID()

  static async import(specifier: string): Promise<JavaScriptRuntime> {
    const self = new this()

    await ops.start(self.#id, specifier)

    run(async () => {
      while (true) {
        const response = await ops.pollDispatchEvent(self.#id)

        if (!response) return

        self.dispatchEvent(
          new EventConstructor[response.constructor](response.type, response.eventInitDict)
        )
      }
    }).catch(unwrap(self))

    return self
  }

  private constructor() {
    super()
  }

  async close(): Promise<void> {
    await ops.close(this.#id)
  }

  postMessage(message: JSONValue): void {
    ops.postMessage(this.#id, message).catch(unwrap(this))
  }
}
