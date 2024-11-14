import { GameObj, Vec2 } from 'kaplay';
import { DARK_BROWN, PLAYER, TOWNS } from '../Globals';
import { townIdx } from '../objects/TownIdx';
import { setupBtnHovers } from '../utils/Button';
import { addButton } from '../utils/Button';
import { highlightToggleable } from '../objects/HighlightToggleable';
import { camScaleUnaffected } from '../utils/CamScaleUnaffected';

scene("mapView", () => {
    setupBtnHovers();

    TOWNS.forEach((town, idx) => {
        addTownMapMarker(idx, town.name, town.position);
    });

    const CAM_PAN_SPEED = 5;
    const CAM_ZOOM_SPEED = 0.01;
    onButtonDown(btn => {
        let disp = vec2();
        if (btn === "map_pan_north") disp = vec2(0, -1);
        if (btn === "map_pan_south") disp = vec2(0, +1);
        if (btn === "map_pan_east") disp = vec2(+1, 0);
        if (btn === "map_pan_west") disp = vec2(-1, 0);
        if (btn === "map_zoom_in") camScale(camScale().scale(1 + CAM_ZOOM_SPEED));
        if (btn === "map_zoom_out") camScale(camScale().scale(1 - CAM_ZOOM_SPEED));
        camPos(camPos().add(disp.scale(CAM_PAN_SPEED)));
    });

    const playerMapMarker = add([
        "playerMapMarker",
        townIdx(PLAYER.townIdx),
        pos(0, 0), // default
        circle(10),
        color(hsl2rgb(0, 1, 1)),
        area(),
        layer("player"),
    ]);

    function currentTown() {
        return get("town").find(town => town.idx === playerMapMarker.idx);
    }

    currentTown()?.addHighlight();
    playerMapMarker.pos = currentTown()?.pos || center();

    onClick("town", newDest => {
        if (newDest.idx === playerMapMarker.idx) return;

        // Remove highlight from town we're leaving
        currentTown()?.removeHighlight();

        // Ensure any previous destinations are unhighlighted
        get("destination").forEach(dest => {
            dest.removeHighlight();
            dest.unuse("destination");
        });

        playerMapMarker.unuse("townIdx");
        playerMapMarker.use("travelling");
        newDest.use("destination");

        const townPos = newDest.pos as Vec2;
        const playerPos = playerMapMarker.pos as Vec2;
        const dir = townPos.sub(playerPos);
        playerMapMarker.use(move(dir, 75.0));
    });

    onCollide("travelling", "destination", (traveller, dest) => {
        traveller.idx = dest.idx;
        traveller.unuse("travelling");
        traveller.unuse("move");
        traveller.pos = dest.pos;

        dest.unuse("destination");
        dest.addHighlight();
        PLAYER.townIdx = dest.idx;
    });


    const enterTownBtn = addButton("Enter Town", {
        tag: "enterTownBtn",
        anchor: "botright",
        pos: pos(width(), height() - 20)
    });


    onClick("enterTownBtn", () => {
        const t = currentTown();
        if (t) {
            go("inTown", t.idx);
        }
    });
});

function addTownMapMarker(idx: number, townName: string, position): GameObj<any> {
    // const townName = randomTownName();
    const FONT_SIZE = 30;
    return add([
        "town",
        named(townName),
        townIdx(idx),
        position,
        circle(16),
        color(
            randi(50, 200),
            randi(50, 200),
            randi(20, 200)
        ),
        area(),
        anchor("center"),
        outline(0, WHITE),
        layer("towns"),
        highlightToggleable(),
        scale(1),
        camScaleUnaffected(),
    ]).add([
        text(townName, {
            size: FONT_SIZE,
            align: "center",
        }),
        color(DARK_BROWN),
        anchor(vec2(0, 4)),
        layer("ui"),
    ]);
};
