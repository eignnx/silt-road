import { Comp } from 'kaplay';

export type CommodityKind = "grain" | "flour" | "feed" | "tools";

export interface Commodity extends Comp {
    id: "commodity";
    kind: CommodityKind;
}
