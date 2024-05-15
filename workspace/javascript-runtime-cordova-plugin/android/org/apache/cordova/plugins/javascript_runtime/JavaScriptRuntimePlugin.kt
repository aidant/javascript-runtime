package org.apache.cordova.plugins.javascript_runtime

import org.apache.cordova.CallbackContext
import org.apache.cordova.CordovaPlugin
import org.apache.cordova.PluginResult

import org.json.JSONArray;

import uniffi.javascript_runtime.Error
import uniffi.javascript_runtime.JavaScriptRuntime

class JavaScriptRuntimePlugin : CordovaPlugin() {
  val runtime: JavaScriptRuntime = JavaScriptRuntime()

  override fun execute(action: String, args: JSONArray, context: CallbackContext): Boolean {
    when (action) {
      "start" -> {
        cordova.getThreadPool().execute(Runnable {
          try {
            this.runtime.start(args.getString(0), args.getString(1))
            context.success()
          } catch (e: Error) {
            context.error(e.toString())
          }
        })
        return true
      }
      "close" -> {
        cordova.getThreadPool().execute(Runnable {
          try {
            this.runtime.close(args.getString(0))
            context.success()
          } catch (e: Error) {
            context.error(e.toString())
          }
        })
        return true
      }
      "postMessage" -> {
        cordova.getThreadPool().execute(Runnable {
          try {
            this.runtime.postMessage(args.getString(0), args.getString(1))
            context.success()
          } catch (e: Error) {
            context.error(e.toString())
          }
        })
        return true
      }
      "pollDispatchEvent" -> {
        cordova.getThreadPool().execute(Runnable {
          try {
            val result = this.runtime.pollDispatchEvent(args.getString(0))
            context.success(result)
          } catch (e: Error) {
            context.error(e.message)
          }
        })
        return true
      }
      else -> {
        context.error("Not implemented on Android.");
        return false
      }
    }
  }
}
