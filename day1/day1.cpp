#include <stdio.h>
#include <iostream>
#include <math.h>
#include <fstream>
using namespace std;

ifstream fin("i.txt");

int main() {
    int mass = 0, p1 = 0, p2 = 0;
    while (fin >> mass) {
        int m = mass / 3 - 2;
        p1 += m;
        int s = m, r = m;
        while (r > 0) {
            r = (r / 3 - 2 > 0) ? r / 3 - 2 : 0;
            s += r;
        }
        p2 += s;
    }
    cout << "PART 1: " << p1 << "\nPART 2: " << p2;
    return 0;
}
