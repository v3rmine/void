var timeElm = document.getElementById('time');
var doc = document.documentElement;
var clientWidth = doc.clientWidth;
var clientHeight = doc.clientHeight;

var width, height, largeHeader, canvas, ctx, target = true;
width = window.innerWidth;
height = window.innerHeight;
target = {x: width/2, y: height/2};

largeHeader = document.getElementById('large-header');
largeHeader.style.height = height+'px';

canvas = document.getElementById('demo-canvas');
canvas.width = width;
canvas.height = height;

// KURO WORK
// URL PARSER
// https://stackoverflow.com/a/901144
function getParameterByName(name, url) {
  if (!url) url = window.location.href;
  name = name.replace(/[\[\]]/g, "\\$&");
  var regex = new RegExp("[?&]" + name + "(=([^&#]*)|&|#|$)"),
      results = regex.exec(url);
  if (!results) return null;
  if (!results[2]) return '';
  return decodeURIComponent(results[2].replace(/\+/g, " "));
}
if (getParameterByName('bg') != null) {
  document.body.style.backgroundImage = "url("+getParameterByName('bg')+")";
}
if (getParameterByName('r') != null) {
  document.body.style.backgroundRepeat = getParameterByName('r');
}
if (getParameterByName('ts') != null) {
  var time = parseInt(getParameterByName('ts'));
} else {
  var time = 0;
}

function timer(timeInSec) {
  var hours = Math.floor(timeInSec / 3600);
  timeInSec = timeInSec - hours * 3600;
  var minutes = Math.floor(timeInSec / 60);
  var seconds = timeInSec - minutes * 60;
  var arr = [hours,minutes,seconds];
  return arr;
}

// COUNTDOWN
// Update the count down every 1 second
var x = setInterval(function() {
  time = time - 1;
  var timing = timer(time);
  // Display the result in the element with id="demo"
  var pad = function pad(val) {
    return val < 10 ? '0' + val : val;
  };
  timeElm.setAttribute('data-hours', pad(timing[0]));
  timeElm.setAttribute('data-minutes', pad(timing[1]));
  timeElm.setAttribute('data-seconds', pad(timing[2]));

  // If the count down is finished, write some text
  if (timing[2] <= 0) {
    clearInterval(x);
    timeElm.setAttribute('data-hours', pad(0));
    timeElm.setAttribute('data-minutes', pad(0));
    timeElm.setAttribute('data-seconds', pad(0));
  }
}, 1000);
