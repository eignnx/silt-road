import kaplay from "kaplay";

kaplay({
    global: true,
    font: "Georgia",
    // letterbox: true,
    width: 800,
    height: 600,
    canvas: document.getElementById("app-canvas") as HTMLCanvasElement,
    buttons: {
        map_pan_north: {
            keyboard: ["w", "up"]
        },
        map_pan_south: {
            keyboard: ["s", "down"]
        },
        map_pan_east: {
            keyboard: ["d", "right"]
        },
        map_pan_west: {
            keyboard: ["a", "left"]
        },
        map_zoom_in: {
            keyboard: ["]", "e"],
        },
        map_zoom_out: {
            keyboard: ["[", "q"],
        },
    }
});

layers(["background", "player", "towns", "ui-bg", "ui"], "ui");
setBackground(hsl2rgb(45 / 360, 0.35, 0.70));
