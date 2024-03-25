function slideshow() {
    // Clone
    const slider1 = document.querySelector('.slider-1');
    const slider2 = slider1.cloneNode(true);
    slider2.classList.remove('slider-1');
    slider2.classList.add('slider-2');
    slider1.parentNode.insertBefore(slider2, slider1.nextSibling);

    // Set first slider
    new Slick(slider1, {
        draggable: false,
        dots: false,
        infinite: true,
        responsive: true,
        asNavFor: slider2,
        touchThreshold: 20,
        speed: 1000,
        fade: true
    });

    // Set second slider
    new Slick(slider2, {
        dots: true,
        infinite: true,
        responsive: true,
        asNavFor: slider1,
        arrows: false,
        speed: 1000,
        easing: 'easeInOutQuart'
    });
}

document.addEventListener('DOMContentLoaded', function() {
    slideshow();
    setTimeout(function() {
        document.querySelector('.slider-1 .slick-next').click();
    }, 1000);
});