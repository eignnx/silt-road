import { KAPLAYCtx, Vec2 } from 'kaplay';
import * as Town from "./Town";

export function plugin(k0: KAPLAYCtx) {
    const k = k0.plug(Town.plugin);

    k.scene("mapView", () => {
        k.setBackground(k.hsl2rgb(45 / 360, 0.35, 0.70));

        for (let i = 0; i < 10; i++) {
            k.spawnTown(i);
        }

        const playerMapMarker = k.add([
            "playerMapMarker",
            Town.townIdx(0),
            k.pos(0, 0), // default
            k.circle(10),
            k.color(k.hsl2rgb(0, 1, 1)),
            k.area(),
            k.z(-2),
        ]);

        function currentTown() {
            return k.get("town").find(town => town.idx === playerMapMarker.idx);
        }

        currentTown()?.toggleHighlight();
        playerMapMarker.pos = currentTown()?.pos || k.center();

        k.onClick("town", newDest => {
            if (newDest.idx === playerMapMarker.idx) return;

            currentTown()?.toggleHighlight();

            playerMapMarker.unuse("townIdx");
            playerMapMarker.use("travelling");
            newDest.use("destination");

            const townPos = newDest.pos as Vec2;
            const playerPos = playerMapMarker.pos as Vec2;
            const dir = townPos.sub(playerPos);
            playerMapMarker.use(k.move(dir, 75.0));
        });

        k.onCollide("travelling", "destination", (traveller, dest) => {
            traveller.idx = dest.idx;
            traveller.unuse("travelling");
            traveller.unuse("move");
            traveller.pos = dest.pos;

            dest.unuse("destination");
            dest.toggleHighlight();
        });
    });

    return {};
}
