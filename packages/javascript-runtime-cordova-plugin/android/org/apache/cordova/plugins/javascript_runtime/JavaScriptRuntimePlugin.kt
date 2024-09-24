package org.apache.cordova.plugins.javascript_runtime

import org.apache.cordova.CallbackContext
import org.apache.cordova.CordovaPlugin
import org.apache.cordova.PluginResult

import org.json.JSONArray;

import uniffi.javascript_runtime.JavaScriptRuntimeException
import uniffi.javascript_runtime.JavaScriptRuntimeImpl

class JavaScriptRuntimePlugin : CordovaPlugin() {
  val runtime: JavaScriptRuntimeImpl = JavaScriptRuntimeImpl()

  override fun execute(action: String, args: JSONArray, context: CallbackContext): Boolean {
    when (action) {
      "start" -> {
        try {
          this.runtime.start(cordova.getActivity().applicationContext.filesDir.absolutePath, args.getString(0), args.getString(1))
          context.success()
        } catch (e: JavaScriptRuntimeException) {
          context.error(e.toString())
          return false
        }
        return true
      }
      "close" -> {
        try {
          this.runtime.close(args.getString(0))
          context.success()
        } catch (e: JavaScriptRuntimeException) {
          context.error(e.toString())
          return false
        }
        return true
      }
      "postMessage" -> {
        try {
          this.runtime.postMessage(args.getString(0), args.getString(1))
          context.success()
        } catch (e: JavaScriptRuntimeException) {
          context.error(e.toString())
          return false
        }
        return true
      }
      "pollDispatchEvent" -> {
        try {
          val result = this.runtime.pollDispatchEvent(args.getString(0))
          context.success(result)
        } catch (e: JavaScriptRuntimeException) {
          context.error(e.toString())
          return false
        }
        return true
      }
      else -> {
        context.error("Not implemented on Android.");
        return false
      }
    }
  }
}
