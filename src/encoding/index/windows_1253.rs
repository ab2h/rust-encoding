// AUTOGENERATED FROM index-windows-1253.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-windows-1253.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    8364, 129, 8218, 402, 8222, 8230, 8224, 8225, 136, 8240, 138, 8249, 140,
    141, 142, 143, 144, 8216, 8217, 8220, 8221, 8226, 8211, 8212, 152, 8482,
    154, 8250, 156, 157, 158, 159, 160, 901, 902, 163, 164, 165, 166, 167, 168,
    169, 65535, 171, 172, 173, 174, 8213, 176, 177, 178, 179, 900, 181, 182,
    183, 904, 905, 906, 187, 908, 189, 910, 911, 912, 913, 914, 915, 916, 917,
    918, 919, 920, 921, 922, 923, 924, 925, 926, 927, 928, 929, 65535, 931,
    932, 933, 934, 935, 936, 937, 938, 939, 940, 941, 942, 943, 944, 945, 946,
    947, 948, 949, 950, 951, 952, 953, 954, 955, 956, 957, 958, 959, 960, 961,
    962, 963, 964, 965, 966, 967, 968, 969, 970, 971, 972, 973, 974, 65535,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        8364 => 128, 129 => 129, 8218 => 130, 402 => 131, 8222 => 132,
        8230 => 133, 8224 => 134, 8225 => 135, 136 => 136, 8240 => 137,
        138 => 138, 8249 => 139, 140 => 140, 141 => 141, 142 => 142,
        143 => 143, 144 => 144, 8216 => 145, 8217 => 146, 8220 => 147,
        8221 => 148, 8226 => 149, 8211 => 150, 8212 => 151, 152 => 152,
        8482 => 153, 154 => 154, 8250 => 155, 156 => 156, 157 => 157,
        158 => 158, 159 => 159, 160 => 160, 901 => 161, 902 => 162, 163 => 163,
        164 => 164, 165 => 165, 166 => 166, 167 => 167, 168 => 168, 169 => 169,
        171 => 171, 172 => 172, 173 => 173, 174 => 174, 8213 => 175,
        176 => 176, 177 => 177, 178 => 178, 179 => 179, 900 => 180, 181 => 181,
        182 => 182, 183 => 183, 904 => 184, 905 => 185, 906 => 186, 187 => 187,
        908 => 188, 189 => 189, 910 => 190, 911 => 191, 912 => 192, 913 => 193,
        914 => 194, 915 => 195, 916 => 196, 917 => 197, 918 => 198, 919 => 199,
        920 => 200, 921 => 201, 922 => 202, 923 => 203, 924 => 204, 925 => 205,
        926 => 206, 927 => 207, 928 => 208, 929 => 209, 931 => 211, 932 => 212,
        933 => 213, 934 => 214, 935 => 215, 936 => 216, 937 => 217, 938 => 218,
        939 => 219, 940 => 220, 941 => 221, 942 => 222, 943 => 223, 944 => 224,
        945 => 225, 946 => 226, 947 => 227, 948 => 228, 949 => 229, 950 => 230,
        951 => 231, 952 => 232, 953 => 233, 954 => 234, 955 => 235, 956 => 236,
        957 => 237, 958 => 238, 959 => 239, 960 => 240, 961 => 241, 962 => 242,
        963 => 243, 964 => 244, 965 => 245, 966 => 246, 967 => 247, 968 => 248,
        969 => 249, 970 => 250, 971 => 251, 972 => 252, 973 => 253, 974 => 254,
        _ => 0
    }
}

#[cfg(test)]
single_byte_tests!()
