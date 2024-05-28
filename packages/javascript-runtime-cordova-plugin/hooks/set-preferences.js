const fs = require('fs')
const path = require('path')

module.exports = exports = () => {
  const lib =
    process.env['JAVASCRIPT_RUNTIME_LIB'] ||
    path.relative(
      path.resolve(__dirname, '..'),
      path.resolve(path.dirname(require.resolve('@javascript-runtime/deno/package.json')), 'lib')
    )
  const profile = process.env['JAVASCRIPT_RUNTIME_PROFILE'] || 'release'

  const filename = path.resolve(__dirname, '../plugin.xml')
  const contents = fs.readFileSync(filename, { encoding: 'utf-8' })

  contents.replace(/(?<=<preference name="JAVASCRIPT_RUNTIME_LIB" default=").*?(?=" \/>)/, lib)
  contents.replace(
    /(?<=<preference name="JAVASCRIPT_RUNTIME_PROFILE" default=").*?(?=" \/>)/,
    profile
  )

  fs.writeFileSync(filename, contents)
}
