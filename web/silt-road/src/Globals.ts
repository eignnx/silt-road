import kaplay, { Vec2, GameObj, Comp, Color, PosComp } from "kaplay";
import "kaplay/global";
import * as TownNames from "./utils/TownNames";
import { ALL_COMMODITIES, CommodityKind } from './objects/Commodity';

export const DARK_BROWN: Color = hsl2rgb(30 / 360, 0.85, 0.1);
export const BROWN: Color = hsl2rgb(30 / 360, 0.85, 0.2);

export interface Town {
    name: string;
    position: PosComp;
    businesses: [string, PosComp][];
    marketInventory: Inventory;
}

export const TOWNS: Town[] = [];

for (let i = 0; i < 10; i++) {

    const marketInventory = {};
    const onHand = chooseMultiple(ALL_COMMODITIES as any, randi(3, ALL_COMMODITIES.length)) as CommodityKind[];
    for (const comm of onHand) {
        marketInventory[comm] = randi(5, 500);
    }

    TOWNS.push({
        name: TownNames.randomTownName(),
        position: pos(randi(width()), randi(height())),
        businesses: [
            ["Wagon Shop", pos(center())],
            ["Market", pos(center().sub(300, 0))],
        ],
        marketInventory,
    });
}

export type Inventory = {
    [K in CommodityKind]?: number;
};

export interface PlayerGlobals {
    townIdx: number;
    inventory: Inventory;
}

export const PLAYER: PlayerGlobals = {
    townIdx: 0,
    inventory: {
        "feed": 123,
        "grain": 4300,
    }
};
