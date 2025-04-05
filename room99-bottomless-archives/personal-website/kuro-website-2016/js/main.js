angular.module('app', ['ui.bootstrap']);

$(window).load(function() { 
	$('#text2').hide();
	$('#text3').hide();
	$('#text4').hide();
	$('body').flowtype({
  		minFont   : 8,
   		maxFont   : 100,
   		fontRatio : 90
	});
});

angular.module('app').controller('ProgressDemoCtrl', function ($scope) {
  $scope.max = 100;
});

function stateSwitch() {
	if(document.body.dataset.state == "disabled" || document.body.dataset.state == "" ) {
		document.body.dataset.state = "enabled";
		$('#textZone').delay(1000).show(3000);

	}
	else {
		document.body.dataset.state = "disabled";
		$('#textZone').hide(1000);
	}
}

$("#js-title").Morphext({
    // The [in] animation type. Refer to Animate.css for a list of available animations.
    animation: "bounceIn",
    // An array of phrases to rotate are created based on this separator. Change it if you wish to separate the phrases differently (e.g. So Simple | Very Doge | Much Wow | Such Cool).
    separator: ",",
    // The delay between the changing of each phrase in milliseconds.
    speed: 2000,
    complete: function () {
        // Called after the entrance animation is executed.
    }
});

$("#tilt").Morphext({
    // The [in] animation type. Refer to Animate.css for a list of available animations.
    animation: "fadeIn",
    // An array of phrases to rotate are created based on this separator. Change it if you wish to separate the phrases differently (e.g. So Simple | Very Doge | Much Wow | Such Cool).
    separator: ",",
    // The delay between the changing of each phrase in milliseconds.
    speed: 1000,
    complete: function () {
        // Called after the entrance animation is executed.
    }
});

function changetext(name,speed) {
	$('#'+name).toggle(speed);
}

//The menu
//contact
$( "#contactLink" ).click(function() {
	changetext('text1',500);
	changetext('text2',1000);

});
$( "#return2" ).click(function() {
	changetext('text1',1000);
	changetext('text2',500);
});
//about
$( "#aboutLink" ).click(function() {
	changetext('text1',500);
	changetext('text3',1000);

});
$( "#return3" ).click(function() {
	changetext('text1',1000);
	changetext('text3',500);
});
//music
$( "#musicLink" ).click(function() {
	changetext('text1',500);
	changetext('text4',1000);
	humane.log("Page in Work!");
});
$( "#return4" ).click(function() {
	changetext('text1',1000);
	changetext('text4',500);
});