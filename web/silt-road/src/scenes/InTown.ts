import { BROWN, DARK_BROWN, TOWNS } from '../Globals';
import { addButton, setupBtnHovers } from '../utils/Button';

scene("inTown", (townIdx) => {
    const town = TOWNS[townIdx];

    setupBtnHovers();

    addButton("Map", {
        tag: "goToMapView",
        anchor: "botright",
        pos: pos(width(), height() - 20)
    });

    onClick("goToMapView", () => {
        go("mapView");
    });

    add([
        text("The town of", { size: 30 }),
        color(DARK_BROWN),
        anchor("top"),
        pos(width() / 2, 50),
    ]).add([
        text(town.name, { size: 60 }),
        color(DARK_BROWN),
        anchor("top"),
        pos(0, 50),
    ]);

    // SPAWN BUSINESSES
    town.businesses.forEach(([name, position], businessIdx) => {
        const w = 200;
        add([
            "businessMarker",
            named(name),
            rect(w, 120),
            color(BROWN),
            position,
            area(),
            outline(5, BLACK),
            anchor("center"),
        ]).add([
            text(name, { width: w, align: "center" }),
            color(WHITE),
            anchor("center"),
        ]);
    });

    onHover("businessMarker", business => {
        business.color = lerp(BROWN, WHITE, 0.05);
    });

    onHoverEnd("businessMarker", business => {
        business.color = BROWN;
    });

    onClick("businessMarker", business => {
        go("businessMenu", business.name);
    });
});
