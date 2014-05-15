// AUTOGENERATED FROM index-iso-8859-3.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-3.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 294, 728, 163, 164, 65535, 292, 167, 168, 304, 350, 286,
    308, 173, 65535, 379, 176, 295, 178, 179, 180, 181, 293, 183, 184, 305,
    351, 287, 309, 189, 65535, 380, 192, 193, 194, 65535, 196, 266, 264, 199,
    200, 201, 202, 203, 204, 205, 206, 207, 65535, 209, 210, 211, 212, 288,
    214, 215, 284, 217, 218, 219, 220, 364, 348, 223, 224, 225, 226, 65535,
    228, 267, 265, 231, 232, 233, 234, 235, 236, 237, 238, 239, 65535, 241,
    242, 243, 244, 289, 246, 247, 285, 249, 250, 251, 252, 365, 349, 729,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        128 => 128, 129 => 129, 130 => 130, 131 => 131, 132 => 132, 133 => 133,
        134 => 134, 135 => 135, 136 => 136, 137 => 137, 138 => 138, 139 => 139,
        140 => 140, 141 => 141, 142 => 142, 143 => 143, 144 => 144, 145 => 145,
        146 => 146, 147 => 147, 148 => 148, 149 => 149, 150 => 150, 151 => 151,
        152 => 152, 153 => 153, 154 => 154, 155 => 155, 156 => 156, 157 => 157,
        158 => 158, 159 => 159, 160 => 160, 294 => 161, 728 => 162, 163 => 163,
        164 => 164, 292 => 166, 167 => 167, 168 => 168, 304 => 169, 350 => 170,
        286 => 171, 308 => 172, 173 => 173, 379 => 175, 176 => 176, 295 => 177,
        178 => 178, 179 => 179, 180 => 180, 181 => 181, 293 => 182, 183 => 183,
        184 => 184, 305 => 185, 351 => 186, 287 => 187, 309 => 188, 189 => 189,
        380 => 191, 192 => 192, 193 => 193, 194 => 194, 196 => 196, 266 => 197,
        264 => 198, 199 => 199, 200 => 200, 201 => 201, 202 => 202, 203 => 203,
        204 => 204, 205 => 205, 206 => 206, 207 => 207, 209 => 209, 210 => 210,
        211 => 211, 212 => 212, 288 => 213, 214 => 214, 215 => 215, 284 => 216,
        217 => 217, 218 => 218, 219 => 219, 220 => 220, 364 => 221, 348 => 222,
        223 => 223, 224 => 224, 225 => 225, 226 => 226, 228 => 228, 267 => 229,
        265 => 230, 231 => 231, 232 => 232, 233 => 233, 234 => 234, 235 => 235,
        236 => 236, 237 => 237, 238 => 238, 239 => 239, 241 => 241, 242 => 242,
        243 => 243, 244 => 244, 289 => 245, 246 => 246, 247 => 247, 285 => 248,
        249 => 249, 250 => 250, 251 => 251, 252 => 252, 365 => 253, 349 => 254,
        729 => 255, _ => 0
    }
}

#[cfg(test)]
single_byte_tests!()
