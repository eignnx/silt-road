import kaplay, { Vec2, GameObj, Comp, Color, PosComp } from "kaplay";
import "kaplay/global";
import { camScaleUnaffected } from './Utils';
import { randomTownName } from './TownNames';

kaplay({
    global: true,
    font: "Georgia",
    letterbox: true,
    width: 1920,
    height: 1080,
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

layers(["player", "towns", "ui"], "ui");
setBackground(hsl2rgb(45 / 360, 0.35, 0.70));

const DARK_BROWN: Color = hsl2rgb(30 / 360, 0.85, 0.1);
const BROWN: Color = hsl2rgb(30 / 360, 0.85, 0.2);

interface Town {
    name: string;
    position: PosComp;
    businesses: [string, PosComp][];
}

const TOWNS: Town[] = [];
for (let i = 0; i < 10; i++) {
    TOWNS.push({
        name: randomTownName(),
        position: pos(randi(width()), randi(height())),
        businesses: [
            ["Wagon Shop", pos(center())],
            ["Market", pos(center().sub(300, 0))],
        ]
    });
}

type Inventory = {
    [K in CommodityKind]?: number;
};

interface PlayerGlobals {
    townIdx: number;
    inventory: Inventory;
}

const PLAYER: PlayerGlobals = {
    townIdx: 0,
    inventory: {
        "feed": 123,
        "grain": 4300,
    }
};

function setupBtnHovers() {
    onHover("button", btn => {
        btn.color = BROWN;
    });
    onHoverEnd("button", btn => {
        btn.color = DARK_BROWN;
    });
}

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

function addButton(txt: string, { tag, anchor: anch, pos: position }): GameObj<any> {
    const w = 24 * txt.length + 10;
    const btn = add([
        tag,
        "button",
        anchor(anch),
        area(),
        position,
        color(BLACK),
        rect(w, 60),
        outline(8, BLACK),
        layer("ui"),
        fixed(),
    ]).add([
        color(WHITE),
        text(txt, { width: w, align: "center" }),
        anchor(anch),
        fixed(),
    ]);

    return btn;
}



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


interface HighlightToggleableComp extends Comp {
    id: "highlightToggleable";
    addHighlight(): void;
    removeHighlight(): void;
}

function highlightToggleable(): HighlightToggleableComp {
    return {
        id: "highlightToggleable",
        inspect() {
            const summary: string[] = [];
            if (this.is("highlighted")) summary.push("highlighted");
            if (this.is("highlightTweening")) summary.push("tweening");
            return `highlight: ${summary}`;
        },
        addHighlight() {
            if (this.is("highlightTweening")) return;
            this.use("highlightTweening");
            tween(1, 7, 1.0,
                val => this.outline.width = val,
                easings.easeOutElastic,
            ).then(() => {
                this.unuse("highlightTweening");
                this.use("highlighted");
            });
        },
        removeHighlight() {
            if (!this.is("highlighted") && !this.is("highlightTweening")) return;
            this.use("highlightTweening");
            tween(7, 1, 2.0,
                val => this.outline.width = val,
                easings.easeOutElastic
            ).then(() => {
                this.unuse("highlighted");
                this.unuse("highlightTweening");
            });
        },
    };
}

export function townIdx(idx: number) {
    return {
        id: "townIdx",
        idx,
        inspect() {
            return `townIdx(${idx})`;
        }
    };
}

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
        scale(2),
        pos(width() / 2, height() * 0.1),
        anchor("center"),
    ]);

    if (businessName === "Market") {

        let position = vec2(width() * 0.15, height() * 0.15);
        for (const comm in PLAYER.inventory) {
            add([
                text(comm, { size: 24, width: width() * 0.8, align: "left" }),
                anchor("left"),
                pos(position),
            ]);
            position = position.add(0, 20);
        }
    } else {
        debug.log(`Business '${businessName}' not implemented!`);
        go("inTown", PLAYER.townIdx);
    }
});

type CommodityKind = "grain" | "flour" | "feed" | "tools";

interface Commodity extends Comp {
    id: "commodity";
    kind: CommodityKind;
}


// go("mapView", PLAYER_TOWN_IDX);
// go("inTown", 0);
go("businessMenu", "Market");
