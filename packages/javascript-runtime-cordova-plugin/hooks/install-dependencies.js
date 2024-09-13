const { spawn } = require('child_process')
const { once } = require('events')
const path = require('path')

module.exports = exports = async (context) => {
  if (context.opts.plugin?.id !== 'cordova-plugin-javascript-runtime') return

  const JAVASCRIPT_RUNTIME_ENGINE =
    process.env['JAVASCRIPT_RUNTIME_ENGINE'] ||
    `https://github.com/aidant/javascript-runtime/releases/download/v1.0/javascript-runtime-deno.tgz`

  const npm = spawn('npm', ['install', '--no-save', '--install-links', JAVASCRIPT_RUNTIME_ENGINE], {
    cwd: path.resolve(__dirname, '..'),
    env: process.env,
    stdio: ['inherit', 'inherit', 'inherit'],
  })

  const [code] = await once(npm, 'exit')

  if (code) {
    throw new Error(`Unable to install the native engine "${JAVASCRIPT_RUNTIME_ENGINE}"`)
  }
}
