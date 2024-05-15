export const run = <Result>(func: () => Result): Result => func()

export const unwrap = (self: EventTarget) => (error: unknown) =>
  self.dispatchEvent(new ErrorEvent('error', { error }))
