// Use most conservative settings for bindgen

#define TARGET_ARCHITECTURE TARGET_ARCHITECTURE_ID_X64 
// #define HACL_CAN_COMPILE_INTRINSICS 1
// #define HACL_CAN_COMPILE_VALE 1
#define HACL_CAN_COMPILE_INLINE_ASM 1
#define Lib_IntVector_Intrinsics_vec128 void *
#define Lib_IntVector_Intrinsics_vec256 void *
// #define HACL_CAN_COMPILE_UINT128 1
#define LINUX_NO_EXPLICIT_BZERO 1
