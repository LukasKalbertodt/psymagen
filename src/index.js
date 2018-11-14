const { incrementing } = wasm_bindgen;

const genImage = (fn, name) => {
    let canvas = document.getElementById('canvas');
    var ctx = canvas.getContext("2d");

    console.log("gen image for: " + name);
    let data = fn();
    ctx.putImageData(data, 0, 0);
}

const init = () => {
    console.log("init...");

    document.getElementById('increment').addEventListener('click', function() {
        genImage(incrementing, 'incrementing');
    });
}

document.addEventListener('DOMContentLoaded', function() {
    wasm_bindgen('./psymagen_bg.wasm').then(init).catch(console.error);
});
