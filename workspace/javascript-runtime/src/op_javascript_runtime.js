import { core, primordials } from 'ext:core/mod.js'
import {
  op_javascript_runtime_poll_dispatch_event,
  op_javascript_runtime_post_message,
} from 'ext:core/ops'
import { CloseEvent, CustomEvent, ErrorEvent, Event, MessageEvent } from 'ext:deno_web/02_event.js'

const { ObjectDefineProperty, ObjectCreate, EventTarget } = primordials

const EventConstructor = {
  Event,
  ErrorEvent,
  CloseEvent,
  MessageEvent,
  CustomEvent,
}

const target = new EventTarget()

;(async () => {
  const promise = op_javascript_runtime_poll_dispatch_event()
  core.unrefOpPromise(promise)
  const response = await promise

  if (!response) return

  target.dispatchEvent(
    new EventConstructor[response.constructor](response.type, response.eventInitDict)
  )
})()

ObjectDefineProperty(
  globalThis,
  'JavaScriptRuntime',
  ObjectCreate(null, {
    addEventListener: core.propWritable(target.addEventListener.bind(target)),
    removeEventListener: core.propWritable(target.removeEventListener.bind(target)),

    postMessage: op_javascript_runtime_post_message.bind(),
  })
)
