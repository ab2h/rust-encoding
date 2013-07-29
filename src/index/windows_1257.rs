// AUTOGENERATED FROM index-windows-1257.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-windows-1257.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    8364, 129, 8218, 131, 8222, 8230, 8224, 8225, 136, 8240, 138, 8249, 140,
    168, 711, 184, 144, 8216, 8217, 8220, 8221, 8226, 8211, 8212, 152, 8482,
    154, 8250, 156, 175, 731, 159, 160, 65535, 162, 163, 164, 65535, 166, 167,
    216, 169, 342, 171, 172, 173, 174, 198, 176, 177, 178, 179, 180, 181, 182,
    183, 248, 185, 343, 187, 188, 189, 190, 230, 260, 302, 256, 262, 196, 197,
    280, 274, 268, 201, 377, 278, 290, 310, 298, 315, 352, 323, 325, 211, 332,
    213, 214, 215, 370, 321, 346, 362, 220, 379, 381, 223, 261, 303, 257, 263,
    228, 229, 281, 275, 269, 233, 378, 279, 291, 311, 299, 316, 353, 324, 326,
    243, 333, 245, 246, 247, 371, 322, 347, 363, 252, 380, 382, 729,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

#[inline]
pub fn backward(code: u16) -> u8 {
    match code {
        8364 => 0, 129 => 1, 8218 => 2, 131 => 3, 8222 => 4, 8230 => 5,
        8224 => 6, 8225 => 7, 136 => 8, 8240 => 9, 138 => 10, 8249 => 11,
        140 => 12, 168 => 13, 711 => 14, 184 => 15, 144 => 16, 8216 => 17,
        8217 => 18, 8220 => 19, 8221 => 20, 8226 => 21, 8211 => 22, 8212 => 23,
        152 => 24, 8482 => 25, 154 => 26, 8250 => 27, 156 => 28, 175 => 29,
        731 => 30, 159 => 31, 160 => 32, 162 => 34, 163 => 35, 164 => 36,
        166 => 38, 167 => 39, 216 => 40, 169 => 41, 342 => 42, 171 => 43,
        172 => 44, 173 => 45, 174 => 46, 198 => 47, 176 => 48, 177 => 49,
        178 => 50, 179 => 51, 180 => 52, 181 => 53, 182 => 54, 183 => 55,
        248 => 56, 185 => 57, 343 => 58, 187 => 59, 188 => 60, 189 => 61,
        190 => 62, 230 => 63, 260 => 64, 302 => 65, 256 => 66, 262 => 67,
        196 => 68, 197 => 69, 280 => 70, 274 => 71, 268 => 72, 201 => 73,
        377 => 74, 278 => 75, 290 => 76, 310 => 77, 298 => 78, 315 => 79,
        352 => 80, 323 => 81, 325 => 82, 211 => 83, 332 => 84, 213 => 85,
        214 => 86, 215 => 87, 370 => 88, 321 => 89, 346 => 90, 362 => 91,
        220 => 92, 379 => 93, 381 => 94, 223 => 95, 261 => 96, 303 => 97,
        257 => 98, 263 => 99, 228 => 100, 229 => 101, 281 => 102, 275 => 103,
        269 => 104, 233 => 105, 378 => 106, 279 => 107, 291 => 108, 311 => 109,
        299 => 110, 316 => 111, 353 => 112, 324 => 113, 326 => 114, 243 => 115,
        333 => 116, 245 => 117, 246 => 118, 247 => 119, 371 => 120, 322 => 121,
        347 => 122, 363 => 123, 252 => 124, 380 => 125, 382 => 126, 729 => 127,
        _ => 255
    }
}
