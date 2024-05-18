const pxScale = () => {
    let scale = {
        px: "1px",
    };

    for (let i = 0; i <= 3000; i++) {
        scale[i] = i + "px";
    }

    return scale;
};

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{rs, html}"],
    theme: {
        spacing: pxScale(),
        maxWidth: pxScale(),
        minWidth: pxScale(),
        maxHeight: pxScale(),
        minHeight: pxScale(),
        extend: {},
    },
    plugins: [],
};
