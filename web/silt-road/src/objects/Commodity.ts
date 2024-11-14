import { Comp } from 'kaplay';


export const ALL_COMMODITIES = [
    "grain", "flour", "feed", "tools", "ammunition", "firearms", "iron ore",
    "textiles", "clothing", "wool", "salt", "cheese"
] as const;

export type CommodityKind = (typeof ALL_COMMODITIES)[number];

export interface Commodity extends Comp {
    id: "commodity";
    kind: CommodityKind;
}
