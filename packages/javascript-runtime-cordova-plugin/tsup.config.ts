import { defineConfig } from 'tsup'

export default defineConfig({
  clean: true,
  dts: true,
  entry: ['src/mod.ts'],
  format: ['cjs', 'esm'],
  outDir: 'lib',
  sourcemap: true,
  splitting: false,
})
