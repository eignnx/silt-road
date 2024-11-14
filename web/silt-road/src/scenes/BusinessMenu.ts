import { DARK_BROWN, PLAYER } from '../Globals';
import { addButton, setupBtnHovers } from '../utils/Button';

scene("businessMenu", businessName => {
    setupBtnHovers();

    // Menu Background
    add([
        rect(width() * 0.9, height() * 0.9),
        color(DARK_BROWN),
        anchor("center"),
        pos(center()),
        outline(25, BLACK, 1.0, "round"),
        layer("background"),
    ]);

    addButton("X", {
        tag: "exitMenuBtn",
        anchor: "topright",
        pos: pos(width() * 0.95, height() * 0.05)
    });


    onClick("exitMenuBtn", () => {
        go("inTown", PLAYER.townIdx);
    });

    add([
        text(businessName, { align: "center", }),
        pos(width() / 2, height() * 0.1),
        anchor("center"),
    ]);

    if (businessName === "Market") {

        add([
            text("Your Cargo", { size: 28, align: "center" }),
            anchor("center"),
            pos(width() / 4, height() * 0.2),
        ]);

        let position = vec2(width() * 0.15, height() * 0.35);
        const rowOpts = { size: 24, width: width() * 0.7 };
        for (const comm in PLAYER.inventory) {

            const qty: string = PLAYER.inventory[comm].toString();
            const txt = comm.padEnd(20, ".") + qty.padStart(10, ".");

            add([
                uiRow(),
                text(txt, { align: "left", ...rowOpts }),
                area(),
                anchor("left"),
                pos(position),
            ]);

            position = position.add(0, 25);
        }


        add([
            text("Market Wares", { size: 28, align: "center" }),
            anchor("center"),
            pos(3 * width() / 4, height() * 0.2),
        ]);


    } else {
        debug.log(`Business '${businessName}' not implemented!`);
        go("inTown", PLAYER.townIdx);
    }

    onHover("uiRow", row => {
        row.use(color(RED));
    });

    onHoverEnd("uiRow", row => {
        row.unuse("color");
    });
});

function uiRow() {
    return {
        id: "uiRow",
        requires: ["area"],
    };
}
