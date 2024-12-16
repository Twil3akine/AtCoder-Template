// MARK: include
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

#define PINF 1000000000000000001
#define NINF -1000000000000000001

#define sh short int
#define ll long long

#define rep(i, v, n) for (ll i = v; i < n; i++)
#define rrep(i, v, n) for (ll i = v - 1; n <= i; i--)
#define drep(i, v, n, d) for (ll i = v; i < n; i += d)

#define max(x, y) ((x >= y) ? (x) : (y))
#define min(x, y) ((x >= y) ? (y) : (x))
#define abs(x) ((x > 0) ? (x) : (-x))

#define len(s) sizeof(s) / sizeof(s[0])

#define ent putchar(10)

ll ipow(const void *x, const void *y)
{
    ll total = 1;
    rep(i, 0, *(ll *)y + 1) total *= *(ll *)x;
    return total;
}

int cmpNumInc(const void *x, const void *y)
{
    if (*(ll *)x < *(ll *)y)
        return -1;
    else if (*(ll *)x > *(ll *)y)
        return 1;
    else
        return 0;
}

int cmpNumDec(const void *x, const void *y)
{
    if (*(ll *)x > *(ll *)y)
        return -1;
    else if (*(ll *)x < *(ll *)y)
        return 1;
    else
        return 0;
}

void isort(ll list[], const ll len)
{
    qsort(list, len, sizeof(list[0]), cmpNumInc);
}

void dsort(ll list[], const ll len)
{
    qsort(list, len, sizeof(list[0]), cmpNumDec);
}

ll lowerBound(const ll list[], const ll len, const ll target)
{
    ll left = 0, right = len;
    while (left < right)
    {
        ll middle = (left + right) / 2;
        if (list[middle] < target)
            left = middle + 1;
        else
            right = middle;
    }
    return right;
}

ll upperBound(const ll list[], const ll len, const ll target)
{
    ll left = 0, right = len;
    while (left < right)
    {
        ll middle = (left + right) / 2;
        if (list[middle] <= target)
            left = middle + 1;
        else
            right = middle;
    }
    return right;
}

void prefixSum(const ll loop, const ll src[], ll dist[])
{
    rep(i, 0, loop)
    {
        dist[i + 1] = dist[i] + src[i];
    }
}

int main(void)
{
    char N[3];
    scanf("%s", &N);
    char a = N[0], b = N[1], c = N[2];

    printf("%c%c%c %c%c%c\n", b, c, a, c, a, b);

    return 0;
}
