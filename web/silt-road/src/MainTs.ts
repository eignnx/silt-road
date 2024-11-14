import kaplay, { Vec2, GameObj, Comp, Color, PosComp } from "kaplay";
import "kaplay/global";

////////////////////////////////////////////////////////////////////////////////
// Load KAPLAY Config:
import "./KaplayConfig";
// Load Assets:
import "./Assets";
// Define Objects:
import "./objects/TownIdx";
import "./objects/HighlightToggleable";
import "./objects/Commodity";
// Define Scenes:
import "./scenes/MapView";
import "./scenes/InTown";
import "./scenes/BusinessMenu";
////////////////////////////////////////////////////////////////////////////////

// go("mapView", PLAYER_TOWN_IDX);
// go("inTown", 0);
go("businessMenu", "Market");
