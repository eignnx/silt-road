import kaplay from "kaplay";
import "kaplay/global";
import * as MapView from "./MapView/MapView"
import * as Commodity from "./Commodity"

const k = kaplay({
    global: false,
    plugins: [
        Commodity.plugin,
        MapView.plugin,
    ],
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
        }
    }
})

const CAM_PAN_SPEED = 3

k.onButtonDown(btn => {
    let disp = vec2();
    if (btn === "map_pan_north") disp = vec2(0, -1)
    if (btn === "map_pan_south") disp = vec2(0, +1)
    if (btn === "map_pan_east") disp = vec2(+1, 0)
    if (btn === "map_pan_west") disp = vec2(-1, 0)
    k.camPos(k.camPos().add(disp.scale(CAM_PAN_SPEED)));
})

k.go("mapView")
