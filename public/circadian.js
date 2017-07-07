var flip = document.querySelector('.flip')
var html = document.querySelector('html')
var stor = window.localStorage

function flipper (night) {
  if (/flipped/.test(html.className) || !night) {
    html.className = html.className.replace('flipped', '')
    flip.innerHTML = 'day'
    flip.title = flip.title.replace('on', 'off')
    stor.setItem('flip', '')
  } else {
    html.className += ' flipped'
    flip.innerHTML = 'night'
    flip.title = flip.title.replace('off', 'on')
    stor.setItem('flip', '✓')
  }
}

flip.addEventListener('click', flipper, null)
flipper(stor.getItem('flip'))
setTimeout(function () {
  html.className += ' js'
}, 1000)
