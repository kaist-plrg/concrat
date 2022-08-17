#include <pthread.h>

int n = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

void f1(int i) {
    if (i <= 0) {
        pthread_mutex_lock(&m);
    } else {
        f1(i - 1);
        n *= i;
    }
}

void f2(int i) {
    if (i <= 0) {
        pthread_mutex_unlock(&m);
    } else {
        n *= i;
        f1(i - 1);
    }
}

void f3() {
    f1(5);
    f2(5);
}

int main() {}
