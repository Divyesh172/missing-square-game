// Re-export bridge to ensure both ./pkg/missing_square_wasm.js and ./pkg/missing_square_game.js resolve seamlessly
import init, * as wasm from "./missing_square_game.js";

export default init;
export * from "./missing_square_game.js";
export { init };
