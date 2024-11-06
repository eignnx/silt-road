import kaplay, { Vec2, GameObj, Comp } from "kaplay";
import "kaplay/global";
import { camScaleUnaffected } from './Utils';
import { randomTownName } from './TownNames';

kaplay({
    global: true,
    font: "sans-serif",
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

scene("mapView", () => {
    layers(["player", "towns", "ui"], "ui");

    setBackground(hsl2rgb(45 / 360, 0.35, 0.70));

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

    for (let i = 0; i < 10; i++) {
        spawnTown(i);
    }

    const playerMapMarker = add([
        "playerMapMarker",
        townIdx(0),
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
    });
});

export function spawnTown(idx: number): GameObj<any> {
    const townName = randomTownName();
    const FONT_SIZE = 30;
    return add([
        "town",
        named(`Town: ${townName}`),
        townIdx(idx),
        pos(rand(width()), rand(height())),
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

const COMMODITY_KINDS = [
    "grain",
    "flour",
    "feed",
    "tools",
];

type CommodityKind = typeof COMMODITY_KINDS[number];

interface Commodity extends Comp {
    id: "commodity";
    kind: CommodityKind;
}

export function commodity(kindOpt?: CommodityKind): Commodity {
    let kind: CommodityKind = kindOpt || choose(COMMODITY_KINDS);
    return {
        id: "commodity",
        kind,
    };
}


go("mapView");
