import { Comp, KAPLAYCtx } from 'kaplay';

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

export function plugin(k: KAPLAYCtx) {
    return {
        hi() {
            k.debug.log("hi");
        }
    };
}
