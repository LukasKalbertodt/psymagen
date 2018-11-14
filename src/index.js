const { incrementing } = wasm_bindgen;

const init = () => {
    console.log("hallU");

    let canvas = document.getElementById('canvas');
    var ctx = canvas.getContext("2d");

    console.log("before");
    let data = incrementing();
    console.log(data);
    console.log("after");
    ctx.putImageData(data, 0, 0);
}

wasm_bindgen('./psymagen_bg.wasm').then(init).catch(console.error);
