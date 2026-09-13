int add(int a, int b);

int multiply(
    int a,
    int b
);

typedef struct
{
    int width;
    int height;
} Size;

extern int public_value;

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

