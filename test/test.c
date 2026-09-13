#include <stdio.h>

#define API_EXPORT


typedef unsigned int uint;

typedef struct
{
    int x;
    int y;
} Point;

struct PublicStruct
{
    int value;
};

union PublicUnion
{
    int integer;
    float decimal;
};

enum PublicKind
{
    KIND_A,
    KIND_B,
    KIND_C
};

extern int external_value;

// @public
int add(int a, int b)
{
    return a + b;
}

// @public
int multiply(
    int a,
    int b
)
{
    return a * b;
}

// @public
typedef struct
{
    int width;
    int height;
} Size;

// @public
extern int public_value;

int private_function(void)
{
    return 42;
}

// @public
struct PublicPoint
{
    int x;
    int y;
};

// @public
enum Status
{
    STATUS_OK,
    STATUS_ERROR
};