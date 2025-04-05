var active = '#center';
var percentW = '80';
var timeout;
var init = {'transform':'translate(0,0)','top':'3vh','left':'2vw'};
$(document).ready(function () {
    $('#top').click(function () {
        if (active === '#center') {$(active).css('opacity','0.5');}
        backInTime(active,'#top');
        if (active !== '#center' && active !== '#top') {
            $(active).css('opacity','0.5');
            if (active === '#left' || active === '#right') {$(active).css('width','7vmin');}
            else {$(active).css('height','7vmin');}
        }
        if (active === '#center') {timeout = 0}
        else {timeout = 400}
        active = '#top';
        setTimeout(function(){
            $('#top').css({
                'height':percentW+'vh',
                'opacity':'1'
            });
            $('#top > h2').css(init);
        },timeout);
    });
    $('#left').click(function () {
        if (active === '#center') {$(active).css('opacity','0.5');}
        backInTime(active, '#left');
        if (active !== '#center' && active !== '#left') {
            $(active).css('opacity','0.5');
            if (active === '#right') {$(active).css('width','7vmin');}
            else {$(active).css('height','7vmin');}
        }
        if (active === '#center') {timeout = 0}
        else {timeout = 400}
        active = '#left';
        setTimeout(function(){$('#left').css({
            'width':percentW+'vw',
            'opacity':'1'
        });
        $('#left > h2').css(init);
        $('#left .content').css('display','block');
        },timeout);
    });
    $('#wrapbehind').click(function () {center()});
    $('#center').click(function() {center()});
    $('#right').click(function () {
        if (active === '#center') {$(active).css('opacity','0.5');}
        backInTime(active, '#right');
        if (active !== '#center' && active !== '#right') {
            $(active).css('opacity','0.5');
            if (active === '#left') {$(active).css('width','7vmin');}
            else {$(active).css('height','7vmin');}
        }
        if (active === '#center') {timeout = 0}
        else {timeout = 400}
        active = '#right';
        setTimeout(function(){$('#right').css({
            'width':percentW+'vw',
            'opacity':'1'
        });
        $('#right > h2').css(init);
        $('#right .content ').css('display','block');
        },timeout);
    });
    $('#bottom').click(function () {
        if (active === '#center') {$(active).css('opacity','0.5');}
        backInTime(active, '#bottom');
        if (active !== '#center' && active !== '#bottom') {
            $(active).css('opacity','0.5');
            if (active === '#left' || active === '#right') {$(active).css('width','7vmin');}
            else {$(active).css('height','7vmin');}
        }
        if (active === '#center') {timeout = 10}
        else {timeout = 400}
        active = '#bottom';
        setTimeout(function(){$('#bottom').css({
            'height':percentW+'vh',
            'opacity':'1'
        });
        $('#bottom > h2').css(init); //'transform','translate(-38vw, 3vh)'
        },timeout);
    });
});

function center() {
    if (active !== '#center') {
        $(active).css('opacity','0.5');
        backInTime(active, null);
        if (active === '#left' || active === '#right') {$(active).css('width','7vmin');}
        else {$(active).css('height','7vmin');}
    }
    active = '#center';
    $(active).css('opacity','1');
}

function backInTime(active, selfi) {
    if (active !== selfi) {
        console.log('back in time MORTYYYY!');
        if (active === '#top') {$('#toptext').css({'transform':'translate(-50%,50%)','top':'0','left':'50%'})}
        else if (active === '#left') {
            $('#lefttext').css({'transform':'rotate(-90deg) translate(0%, 100%)','top':'50%','right':'-125%', 'left':'unset'});
            $('#left .content').css('display','none')
        }
        else if (active === '#right') {
            $('#righttext').css({'transform':'rotate(90deg) translate(0%, 300%)','top':'50%','left':'0'});
            $('#right .content').css('display','none')
        }
        else if (active === '#bottom') {$('#bottomtext').css({'transform':'translate(-50%,50%)','top':'0','bottom':'3vh','left':'50%'})}
    }
}
