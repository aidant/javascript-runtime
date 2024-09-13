const fs = require('fs/promises')
const path = require('path')

module.exports = exports = async (context) => {
  if (context.opts.plugin && context.opts.plugin.id !== 'cordova-plugin-javascript-runtime') return

  const lib =
    process.env['JAVASCRIPT_RUNTIME_LIB'] ||
    path.relative(
      path.resolve(__dirname, '..'),
      path.resolve(path.dirname(require.resolve('@javascript-runtime/deno/package.json')), 'lib')
    )
  const profile = process.env['JAVASCRIPT_RUNTIME_PROFILE'] || 'release'

  const filename = path.resolve(__dirname, '../plugin.xml')
  const contents = await fs.readFile(filename, { encoding: 'utf-8' })

  const newContents = contents
    .replace(
      /(?<=src=").*?(?=" target-dir="java\/uniffi\/javascript_runtime\/")/,
      `${lib}/android/uniffi/javascript_runtime/javascript_runtime.kt`
    )
    .replace(
      /(?<=src=").*?(?=" target="jniLibs\/arm64-v8a\/libjavascript_runtime\.so")/,
      `${lib}/android/uniffi/javascript_runtime/aarch64-linux-android/${profile}/libjavascript_runtime.so`
    )
    .replace(
      /(?<=src=").*?(?=" target="jniLibs\/x86_64\/libjavascript_runtime\.so")/,
      `${lib}/android/uniffi/javascript_runtime/x86_64-linux-android/${profile}/libjavascript_runtime.so`
    )

  await fs.writeFile(filename, newContents)
}
