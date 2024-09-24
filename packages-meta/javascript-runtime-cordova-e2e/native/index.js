fetch('https://1.1.1.1')
  .then((response) => response.text())
  .then(console.log, console.error)
