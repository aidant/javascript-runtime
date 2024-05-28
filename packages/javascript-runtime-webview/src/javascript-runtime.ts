import { EventConstructor, ops, type JSONValue } from './ops.js'

export class JavaScriptRuntime extends EventTarget {
  static async import(specifier: string): Promise<JavaScriptRuntime> {
    const self = new this()

    await ops.start(self.#id, specifier)

    const pollDispatchEvent = async () => {
      while (true) {
        const response = await ops.pollDispatchEvent(self.#id)

        if (!response) return

        self.dispatchEvent(
          new EventConstructor[response.constructor](response.type, response.eventInitDict)
        )
      }
    }

    pollDispatchEvent().catch((error) => {
      self.dispatchEvent(new ErrorEvent('error', { error }))
    })

    return self
  }

  #id = crypto.randomUUID()

  // @ts-expect-error
  private constructor()

  async close(): Promise<void> {
    await ops.close(this.#id)
  }

  postMessage(message: JSONValue): void {
    ops.postMessage(this.#id, message).catch((error) => {
      self.dispatchEvent(
        new MessageEvent('messageerror', {
          data: message,
          source: this as EventTarget as MessageEventSource,
        })
      )
    })
  }
}
