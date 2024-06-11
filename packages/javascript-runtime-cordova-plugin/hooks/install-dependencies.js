const { spawn } = require('child_process')
const { once } = require('events')
const path = require('path')

module.exports = exports = async () => {
  const JAVASCRIPT_RUNTIME_ENGINE =
    process.env['JAVASCRIPT_RUNTIME_ENGINE'] || '@javascript-runtime/deno'

  const npm = spawn('npm', ['install', '--no-save', JAVASCRIPT_RUNTIME_ENGINE], {
    cwd: path.resolve(__dirname, '..'),
    env: process.env,
    stdio: ['inherit', 'inherit', 'inherit'],
  })

  const [code] = await once(npm, 'exit')

  if (code) {
    throw new Error(`Unable to install the native engine "${JAVASCRIPT_RUNTIME_ENGINE}"`)
  }
}
