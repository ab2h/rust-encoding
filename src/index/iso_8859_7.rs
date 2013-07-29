// AUTOGENERATED FROM index-iso-8859-7.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-7.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 8216, 8217, 163, 8364, 8367, 166, 167, 168, 169, 890, 171,
    172, 173, 65535, 8213, 176, 177, 178, 179, 900, 901, 902, 183, 904, 905,
    906, 187, 908, 189, 910, 911, 912, 913, 914, 915, 916, 917, 918, 919, 920,
    921, 922, 923, 924, 925, 926, 927, 928, 929, 65535, 931, 932, 933, 934,
    935, 936, 937, 938, 939, 940, 941, 942, 943, 944, 945, 946, 947, 948, 949,
    950, 951, 952, 953, 954, 955, 956, 957, 958, 959, 960, 961, 962, 963, 964,
    965, 966, 967, 968, 969, 970, 971, 972, 973, 974, 65535,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

#[inline]
pub fn backward(code: u16) -> u8 {
    match code {
        128 => 0, 129 => 1, 130 => 2, 131 => 3, 132 => 4, 133 => 5, 134 => 6,
        135 => 7, 136 => 8, 137 => 9, 138 => 10, 139 => 11, 140 => 12,
        141 => 13, 142 => 14, 143 => 15, 144 => 16, 145 => 17, 146 => 18,
        147 => 19, 148 => 20, 149 => 21, 150 => 22, 151 => 23, 152 => 24,
        153 => 25, 154 => 26, 155 => 27, 156 => 28, 157 => 29, 158 => 30,
        159 => 31, 160 => 32, 8216 => 33, 8217 => 34, 163 => 35, 8364 => 36,
        8367 => 37, 166 => 38, 167 => 39, 168 => 40, 169 => 41, 890 => 42,
        171 => 43, 172 => 44, 173 => 45, 8213 => 47, 176 => 48, 177 => 49,
        178 => 50, 179 => 51, 900 => 52, 901 => 53, 902 => 54, 183 => 55,
        904 => 56, 905 => 57, 906 => 58, 187 => 59, 908 => 60, 189 => 61,
        910 => 62, 911 => 63, 912 => 64, 913 => 65, 914 => 66, 915 => 67,
        916 => 68, 917 => 69, 918 => 70, 919 => 71, 920 => 72, 921 => 73,
        922 => 74, 923 => 75, 924 => 76, 925 => 77, 926 => 78, 927 => 79,
        928 => 80, 929 => 81, 931 => 83, 932 => 84, 933 => 85, 934 => 86,
        935 => 87, 936 => 88, 937 => 89, 938 => 90, 939 => 91, 940 => 92,
        941 => 93, 942 => 94, 943 => 95, 944 => 96, 945 => 97, 946 => 98,
        947 => 99, 948 => 100, 949 => 101, 950 => 102, 951 => 103, 952 => 104,
        953 => 105, 954 => 106, 955 => 107, 956 => 108, 957 => 109, 958 => 110,
        959 => 111, 960 => 112, 961 => 113, 962 => 114, 963 => 115, 964 => 116,
        965 => 117, 966 => 118, 967 => 119, 968 => 120, 969 => 121, 970 => 122,
        971 => 123, 972 => 124, 973 => 125, 974 => 126, _ => 255
    }
}
