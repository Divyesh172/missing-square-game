/* tslint:disable */
/* eslint-disable */

export enum ConfigurationType {
    Original = 0,
    Rearranged = 1,
    Reset = 2,
}

/**
 * Identifiers for the four classic Curry dissection pieces.
 */
export enum PieceId {
    RedTriangle = 0,
    GreenTriangle = 1,
    OrangePolyomino = 2,
    YellowPolyomino = 3,
}

/**
 * 2D Point representation with high-precision 64-bit floats.
 */
export class Point {
    free(): void;
    [Symbol.dispose](): void;
    distance(other: Point): number;
    constructor(x: number, y: number);
    translate(dx: number, dy: number): Point;
    x: number;
    y: number;
}

export class PuzzleEngine {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Clears any selected piece.
     */
    deselect_piece(): void;
    /**
     * Updates the position of a piece during interactive dragging.
     */
    drag_move(id: string, target_x: number, target_y: number): boolean;
    /**
     * Moves a piece by a relative offset (dx, dy).
     */
    drag_offset(id: string, dx: number, dy: number): boolean;
    /**
     * Evaluates Cassini's Identity for n=6 and returns JSON.
     */
    get_cassini_info(): string;
    /**
     * Returns an in-depth mathematical explanation of the Curry Missing Square illusion.
     */
    get_mathematical_explanation(): string;
    /**
     * Returns a JSON array of all piece states including position, area, slope, and color.
     */
    get_piece_positions_json(): string;
    /**
     * Returns the currently selected piece identifier, if any.
     */
    get_selected_piece(): string | undefined;
    /**
     * Returns a friendly human-readable summary of the current puzzle state.
     */
    get_status_message(): string;
    is_laser_visible(): boolean;
    is_lozenge_visible(): boolean;
    is_magnify_bend(): boolean;
    constructor();
    /**
     * Renders the complete vector SVG board based on current state and options.
     */
    render_svg(): string;
    /**
     * Renders SVG board with ad-hoc option flags.
     */
    render_svg_with_options(laser: boolean, lozenge: boolean, magnify: boolean): string;
    /**
     * Resets the puzzle to Configuration A.
     */
    reset(): void;
    /**
     * Runs geometric and mathematical validation on the current piece positions.
     * Returns a JSON string containing the full `ValidationReport`.
     */
    run_validation(): string;
    /**
     * Selects a piece by its identifier ("red", "green", "orange", "yellow", or enum name).
     */
    select_piece(id: string): boolean;
    /**
     * Sets all pieces to a canonical configuration ("A", "B", or "reset").
     */
    set_configuration(config: string): boolean;
    /**
     * Sets configuration by ConfigurationType enum.
     */
    set_configuration_type(config: ConfigurationType): boolean;
    set_laser_visible(visible: boolean): void;
    set_lozenge_visible(visible: boolean): void;
    set_magnify_bend(magnify: boolean): void;
    /**
     * Sets piece position directly.
     */
    set_piece_position(id: string, x: number, y: number): boolean;
    /**
     * Magnetically snaps all pieces on the board.
     */
    snap_all_pieces(): void;
    /**
     * Magnetically snaps the specified piece to the nearest valid grid coordinate or canonical slot.
     */
    snap_selected_piece(id: string): boolean;
    /**
     * Toggles visibility of the laser straightedge hypotenuse.
     */
    toggle_laser(): boolean;
    /**
     * Toggles visibility of the mystery lozenge strip.
     */
    toggle_lozenge(): boolean;
    /**
     * Toggles dynamic bend magnification loupe.
     */
    toggle_magnification(): boolean;
}

export function main_js(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_get_point_x: (a: number) => number;
    readonly __wbg_get_point_y: (a: number) => number;
    readonly __wbg_point_free: (a: number, b: number) => void;
    readonly __wbg_puzzleengine_free: (a: number, b: number) => void;
    readonly __wbg_set_point_x: (a: number, b: number) => void;
    readonly __wbg_set_point_y: (a: number, b: number) => void;
    readonly main_js: () => void;
    readonly point_distance: (a: number, b: number) => number;
    readonly point_new: (a: number, b: number) => number;
    readonly point_translate: (a: number, b: number, c: number) => number;
    readonly puzzleengine_deselect_piece: (a: number) => void;
    readonly puzzleengine_drag_move: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly puzzleengine_drag_offset: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly puzzleengine_get_cassini_info: (a: number) => [number, number];
    readonly puzzleengine_get_mathematical_explanation: (a: number) => [number, number];
    readonly puzzleengine_get_piece_positions_json: (a: number) => [number, number];
    readonly puzzleengine_get_selected_piece: (a: number) => [number, number];
    readonly puzzleengine_get_status_message: (a: number) => [number, number];
    readonly puzzleengine_is_laser_visible: (a: number) => number;
    readonly puzzleengine_is_lozenge_visible: (a: number) => number;
    readonly puzzleengine_is_magnify_bend: (a: number) => number;
    readonly puzzleengine_new: () => number;
    readonly puzzleengine_render_svg: (a: number) => [number, number];
    readonly puzzleengine_render_svg_with_options: (a: number, b: number, c: number, d: number) => [number, number];
    readonly puzzleengine_reset: (a: number) => void;
    readonly puzzleengine_run_validation: (a: number) => [number, number];
    readonly puzzleengine_select_piece: (a: number, b: number, c: number) => number;
    readonly puzzleengine_set_configuration: (a: number, b: number, c: number) => number;
    readonly puzzleengine_set_configuration_type: (a: number, b: number) => number;
    readonly puzzleengine_set_laser_visible: (a: number, b: number) => void;
    readonly puzzleengine_set_lozenge_visible: (a: number, b: number) => void;
    readonly puzzleengine_set_magnify_bend: (a: number, b: number) => void;
    readonly puzzleengine_set_piece_position: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly puzzleengine_snap_all_pieces: (a: number) => void;
    readonly puzzleengine_snap_selected_piece: (a: number, b: number, c: number) => number;
    readonly puzzleengine_toggle_laser: (a: number) => number;
    readonly puzzleengine_toggle_lozenge: (a: number) => number;
    readonly puzzleengine_toggle_magnification: (a: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
