#include <iostream>
#include <complex>
#include <ctime>
#include <vector>

#define M 1024
#define N 1024
#define P 1024

using namespace std;

complex<float> genRandomComplex() {
    float randomFloat = float(rand()) / float(RAND_MAX) * 50.0;
    complex<float> c(randomFloat, randomFloat*5.2775);
    return c;
}

class Matrice {
    public:
        Matrice(int width, int height) {
            this->width = width;
            this->height = height;
            
            this->matrix = vector<vector<complex<float> > >(width);
                for (int i = 0; i < width; i++) {
                    this->matrix[i] = vector<complex<float> >(height);
                }
        }

        void generateRandoms() {
            for (int x = 0; x < this->width; x++) {
                for (int y = 0; y < this->height; y++) {
                    this->matrix[x][y] = genRandomComplex();
                }
            }
        }

        complex<float> getValue(int x, int y) {
            return this->matrix[x][y];
        }

        void setValue(int x, int y, complex<float> c) {
            this->matrix[x][y] = c;
        }

    private:
        int width, height;
        vector<vector<complex<float> > > matrix;
};

int main() {
    srand((unsigned int)time(NULL));

    Matrice a(N, M), b(M, P), c(N, P);
    a.generateRandoms();
    b.generateRandoms();
    c.generateRandoms();

    for (int x = 0; x < N; x++) {
        for (int y = 0; y < P; y++) {
            for (int k = 0; k < M; k++) {
                c.setValue(x, y, a.getValue(x, k) * b.getValue(k, y));
            }
        }
    }

    cout << c.getValue(N/2, P/2) << endl;
}
