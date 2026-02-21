import {
  stringBuilderAppendCodePoint as stringBuilderAppendCodePoint_353
} from "@temperlang/core";
/**
 * @param {number} n_349
 * @param {globalThis.Array<string>} sb_350
 */
function appendHex_348(n_349, sb_350) {
  let t_351;
  const i_352 = n_349 & 15;
  try {
    if (i_352 < 10) {
      t_351 = 48;
    } else {
      t_351 = 87;
    }
    stringBuilderAppendCodePoint_353(sb_350, i_352 + t_351 | 0);
  } catch {
    throw Error();
  }
  return;
}
/**
 * @param {number} octet_354
 * @param {globalThis.Array<string>} sb_355
 */
export function percentEscapeOctetTo(octet_354, sb_355) {
  sb_355[0] += "%";
  let t_356 = (octet_354 & 255) / 16 | 0;
  appendHex_348(t_356, sb_355);
  let t_357 = octet_354 & 15;
  appendHex_348(t_357, sb_355);
  return;
};
