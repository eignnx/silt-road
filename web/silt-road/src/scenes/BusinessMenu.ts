import { PosComp, Vec2 } from 'kaplay';
import { DARK_BROWN, Inventory, PLAYER, TOWNS } from '../Globals';
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

        function addInventoryDisplay(
            position: Vec2,
            title: string,
            inventory: Inventory,
            widthChars: number
        ) {
            add([
                text(title, { size: 28, align: "center" }),
                anchor("left"),
                pos(position),
            ]);

            position = position.add(0, 35);


            for (const comm in inventory) {
                const qty: string = inventory[comm].toString();
                const txt = comm.padEnd(widthChars / 2, ".") + qty.padStart(widthChars / 2, ".");

                const txtObj = add([
                    uiRow(),
                    text(txt, {
                        size: 20,
                        align: "left",
                        font: "monospace"
                    }),
                    area(),
                    anchor("left"),
                    pos(position),
                ]);

                addButton("buy", {
                    tag: "buy",
                    anchor: "left",
                    pos: pos(position.add(txtObj.width + 10, 0)),
                    fontSize: 18,
                });

                addButton("sell", {
                    tag: "sell",
                    anchor: "left",
                    pos: pos(position.add(txtObj.width + 55, 0)),
                    fontSize: 18,
                });

                position = position.add(0, 35);
            }
        }

        const widthChars = 16;

        addInventoryDisplay(
            vec2(width() * 0.10, height() * 0.3),
            "Your Cargo",
            PLAYER.inventory,
            widthChars,
        );


        addInventoryDisplay(
            vec2(width() * 0.55, height() * 0.3),
            "Market Wares",
            TOWNS[PLAYER.townIdx].marketInventory,
            widthChars,
        );


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
