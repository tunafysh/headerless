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

int add(int a, int b)
{
return a + b;
}

int multiply(
    int a,
    int b
)
{
return a * b;
}

typedef struct
{
    int width;
    int height;
} Size;

extern int public_value;

int private_function(void)
{
return 42;
}

struct PublicPoint
{
    int x;
    int y;
};

enum Status
{
    STATUS_OK,
    STATUS_ERROR
};
